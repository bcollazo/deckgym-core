use std::{collections::HashMap, panic};

use log::debug;
use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::StdRng};

use crate::{
    actions::effect_ability_mechanic_map::{get_ability_mechanic, has_ability_mechanic},
    actions::{
        abilities::AbilityMechanic,
        apply_abilities_action::forecast_ability,
        apply_action_helpers::{apply_activate, wrap_with_common_logic},
    },
    effects::TurnEffect,
    hooks::{
        get_retreat_cost, on_bench_from_hand, on_evolve, to_playable_card, DamageModifierContext,
    },
    models::{Card, EnergyType},
    state::State,
    tools,
};

use super::{
    apply_action_helpers::{
        enumerate_guts_survivor_subsets, forecast_end_turn, guts_flipping_targets, handle_damage,
        handle_damage_only, handle_knockouts, knock_out_attacker_for_perish_body,
        perish_body_flips, set_guts_survivors_to_10, Mutation, Mutations,
    },
    apply_attack_action::forecast_attack,
    apply_stadium_action::{self, forecast_use_stadium},
    apply_trainer_action::forecast_trainer_action,
    outcomes::Outcomes,
    Action, SimpleAction,
};

/// Main function to mutate the state based on the action. It forecasts the possible outcomes
/// and then chooses one of them to apply. This is so that bot implementations can re-use the
/// `forecast_action` function.
pub fn apply_action(rng: &mut StdRng, state: &mut State, action: &Action) {
    let (probabilities, mut lazy_mutations) = forecast_action(state, action).into_branches();
    if probabilities.len() == 1 {
        lazy_mutations.remove(0)(rng, state, action);
    } else {
        let dist = WeightedIndex::new(&probabilities).unwrap();
        let chosen_index = dist.sample(rng);
        lazy_mutations.remove(chosen_index)(rng, state, action);
    }
}

/// This should be mostly a "router" function that calls the appropriate forecast function
/// based on the action type.
pub fn forecast_action(state: &State, action: &Action) -> Outcomes {
    let mut outcomes = match &action.action {
        // Deterministic Actions
        SimpleAction::DrawCard { .. } // TODO: DrawCard should return actual deck probabilities.
        | SimpleAction::Place(_, _)
        | SimpleAction::Attach { .. }
        | SimpleAction::MoveEnergy { .. }
        | SimpleAction::AttachTool { .. }
        | SimpleAction::Evolve { .. }
        | SimpleAction::Activate { .. }
        | SimpleAction::Retreat(_)
        | SimpleAction::ScheduleDelayedSpotDamage { .. }
        | SimpleAction::Heal { .. }
        | SimpleAction::HealAndDiscardEnergy { .. }
        | SimpleAction::MoveAllDamage { .. }
        | SimpleAction::ApplyEeveeBagDamageBoost
        | SimpleAction::HealAllEeveeEvolutions
        | SimpleAction::DiscardFossil { .. }
        | SimpleAction::ReturnPokemonToHand { .. }
        | SimpleAction::ShuffleInPlayPokemonIntoDeck { .. }
        | SimpleAction::DiscardToolFromPokemon { .. }
        | SimpleAction::DiscardActiveStadium
        | SimpleAction::DiscardRandomOpponentActiveEnergy
        | SimpleAction::ApplyStatusToOpponentActive { .. }
        | SimpleAction::Noop => forecast_deterministic_action(),
        SimpleAction::UseAbility { in_play_idx } => forecast_ability(state, action, *in_play_idx),
        SimpleAction::ApplyDamage {
            attacking_ref,
            targets,
            is_from_active_attack,
        } => forecast_apply_damage(state, *attacking_ref, targets, *is_from_active_attack),
        SimpleAction::Attack(attack) => {
            forecast_attack(action.actor, state, attack, action.is_stack)
        }
        SimpleAction::Play { trainer_card } => {
            forecast_trainer_action(action.actor, state, trainer_card)
        }
        SimpleAction::CommunicatePokemon { hand_pokemon } => {
            forecast_pokemon_communication(action.actor, state, hand_pokemon)
        }
        SimpleAction::ShufflePokemonIntoDeck { hand_pokemon } => {
            forecast_shuffle_pokemon_into_deck(action.actor, hand_pokemon)
        }
        SimpleAction::ShuffleOwnCardsIntoDeck { cards } => {
            forecast_shuffle_own_cards_into_deck(action.actor, cards)
        }
        SimpleAction::SwitchHandCardForRandomTool { hand_card } => {
            apply_stadium_action::forecast_switch_hand_card_for_random_tool(
                state,
                action.actor,
                hand_card,
            )
        }
        SimpleAction::ShuffleOpponentSupporter { supporter_card } => {
            forecast_shuffle_opponent_supporter(action.actor, supporter_card)
        }
        SimpleAction::DiscardOpponentSupporter { supporter_card } => {
            forecast_discard_opponent_supporter(action.actor, supporter_card)
        }
        SimpleAction::DiscardOwnCards { cards } => forecast_discard_own_cards(action.actor, cards),
        SimpleAction::AttachFromDiscard {
            in_play_idx,
            num_random_energies,
        } => forecast_attach_from_discard(
            state,
            action.actor,
            *in_play_idx,
            *num_random_energies,
        ),
        SimpleAction::AttachTypedFromDiscard {
            in_play_idx,
            energy_type,
            count,
        } => forecast_attach_typed_from_discard(*in_play_idx, *energy_type, *count),
        SimpleAction::SadaAttach { assignments } => forecast_sada_attach(assignments),
        SimpleAction::UseStadium => forecast_use_stadium(state, action.actor),
        // acting_player is not passed here, because there is only 1 turn to end. The current turn.
        SimpleAction::EndTurn => {
            let (probabilities, mutations) = forecast_end_turn(state);
            Outcomes::from_parts(probabilities, mutations)
        }
    };

    // This is where we basically "apply" Will in a way that is forecasteable.
    // (The player should know if they have an upcoming Will).
    if is_will_eligible_action(&action.action) && state.has_pending_will_first_heads() {
        outcomes = match outcomes.force_first_heads() {
            Ok(forced_outcomes) => forced_outcomes.map_mutations(|mutation| {
                Box::new(move |rng, state, action| {
                    state.consume_pending_will_first_heads();
                    mutation(rng, state, action);
                })
            }),
            Err(original_outcomes) => original_outcomes,
        };
    }

    // Wrap with common logic for mutations
    outcomes.map_mutations(wrap_with_common_logic)
}

fn is_will_eligible_action(action: &SimpleAction) -> bool {
    matches!(
        action,
        SimpleAction::Attack(_)
            | SimpleAction::UseAbility { .. }
            | SimpleAction::Play { .. }
            | SimpleAction::UseStadium
    )
}

fn forecast_deterministic_action() -> Outcomes {
    Outcomes::single_fn(move |_, state, action| {
        apply_deterministic_action(state, action);
    })
}

/// ApplyDamage (damage queued through the move-generation stack, e.g. Mega Kangaskhan's second
/// punch or Raikou ex's spot damage) is deterministic unless a target has the Guts ability and
/// would be knocked out: each such target flips its own survival coin, independently of any
/// Guts flip already resolved earlier in the same attack.
fn forecast_apply_damage(
    state: &State,
    attacking_ref: (usize, usize),
    targets: &[(u32, usize, usize)],
    is_from_active_attack: bool,
) -> Outcomes {
    let context = DamageModifierContext {
        attack_name: None,
        attack_effect: None,
    };
    let flipping = guts_flipping_targets(
        state,
        attacking_ref,
        targets,
        is_from_active_attack,
        context,
    );
    let perish = perish_body_flips(
        state,
        attacking_ref,
        targets,
        is_from_active_attack,
        context,
    );

    if flipping.is_empty() && !perish {
        let targets = targets.to_vec();
        return Outcomes::single_fn(move |_, state, _| {
            handle_damage(state, attacking_ref, &targets, is_from_active_attack, None);
        });
    }

    // One branch per heads/tails combination (Guts flips are independent of the defender's
    // Perish Body flip); on heads the damage still applies (so on-damage triggers fire) and
    // the Guts survivor's remaining HP is set to 10 before knockouts resolve, while a Perish
    // Body heads knocks out the attacker after knockouts resolve.
    let subsets = enumerate_guts_survivor_subsets(&flipping);
    let perish_options: &[bool] = if perish { &[true, false] } else { &[false] };
    let combos = subsets.len() * perish_options.len();
    let probabilities = vec![1.0 / combos as f64; combos];
    let mut mutations: Mutations = vec![];
    for survivors in subsets {
        for perish_heads in perish_options.iter().copied() {
            let survivors = survivors.clone();
            let targets = targets.to_vec();
            mutations.push(
                Box::new(move |_: &mut StdRng, state: &mut State, _: &Action| {
                    handle_damage_only(
                        state,
                        attacking_ref,
                        &targets,
                        is_from_active_attack,
                        DamageModifierContext {
                            attack_name: None,
                            attack_effect: None,
                        },
                    );
                    set_guts_survivors_to_10(state, &survivors);
                    // Perish Body only fires if the defending Active Pokémon actually got knocked
                    // out; capture that before `handle_knockouts` discards it.
                    let defender = (attacking_ref.0 + 1) % 2;
                    let perish_armed = perish_heads
                        && state.in_play_pokemon[defender][0]
                            .as_ref()
                            .is_some_and(|pokemon| pokemon.is_knocked_out());
                    handle_knockouts(state, attacking_ref, is_from_active_attack);
                    if perish_armed {
                        knock_out_attacker_for_perish_body(state, attacking_ref);
                    }
                }) as Mutation,
            );
        }
    }
    Outcomes::from_parts(probabilities, mutations)
}

fn apply_deterministic_action(state: &mut State, action: &Action) {
    match &action.action {
        SimpleAction::DrawCard { amount } => {
            for _ in 0..*amount {
                state.maybe_draw_card(action.actor);
            }
        }
        SimpleAction::Attach {
            attachments,
            is_turn_energy,
        } => apply_attach_energy(state, action.actor, attachments, *is_turn_energy),
        SimpleAction::AttachTool {
            in_play_idx,
            tool_card,
        } => apply_attach_tool(state, action.actor, *in_play_idx, tool_card),
        SimpleAction::MoveEnergy {
            from_in_play_idx,
            to_in_play_idx,
            energy_type,
            amount,
        } => apply_move_energy(
            state,
            action.actor,
            *from_in_play_idx,
            *to_in_play_idx,
            *energy_type,
            *amount,
        ),
        SimpleAction::Place(card, index) => {
            apply_place_card(state, action.actor, card, *index, false)
        }
        SimpleAction::Evolve {
            evolution,
            in_play_idx,
            from_deck,
        } => apply_evolve(action.actor, state, evolution, *in_play_idx, *from_deck),
        SimpleAction::Activate {
            player,
            in_play_idx,
        } => apply_retreat(*player, state, *in_play_idx, true),
        SimpleAction::Retreat(position) => apply_retreat(action.actor, state, *position, false),
        SimpleAction::ScheduleDelayedSpotDamage {
            target_player,
            target_in_play_idx,
            amount,
        } => apply_schedule_delayed_spot_damage(
            state,
            action.actor,
            *target_player,
            *target_in_play_idx,
            *amount,
        ),
        // Trainer-Specific Actions
        SimpleAction::Heal {
            in_play_idx,
            amount,
            cure_status,
        } => apply_healing(action.actor, state, *in_play_idx, *amount, *cure_status),
        SimpleAction::HealAndDiscardEnergy {
            in_play_idx,
            heal_amount,
            discard_energies,
        } => apply_heal_and_discard_energy(
            action.actor,
            state,
            *in_play_idx,
            *heal_amount,
            discard_energies,
        ),
        SimpleAction::MoveAllDamage { from, to } => {
            apply_move_all_damage(action.actor, state, *from, *to)
        }
        SimpleAction::ApplyEeveeBagDamageBoost => apply_eevee_bag_damage_boost(state),
        SimpleAction::HealAllEeveeEvolutions => {
            apply_heal_all_eevee_evolutions(action.actor, state)
        }
        SimpleAction::DiscardFossil { in_play_idx } => {
            apply_discard_fossil(action.actor, state, *in_play_idx)
        }
        SimpleAction::ReturnPokemonToHand { in_play_idx } => {
            apply_return_pokemon_to_hand(action.actor, state, *in_play_idx)
        }
        SimpleAction::ShuffleInPlayPokemonIntoDeck { in_play_idx } => {
            apply_shuffle_in_play_pokemon_into_deck(action.actor, state, *in_play_idx)
        }
        SimpleAction::DiscardToolFromPokemon {
            player,
            in_play_idx,
        } => {
            state.discard_tool(*player, *in_play_idx);
        }
        SimpleAction::DiscardActiveStadium => {
            if let Some((stadium, owner)) = state.take_active_stadium() {
                state.discard_piles[owner.unwrap_or(action.actor)].push(stadium);
            }
        }
        SimpleAction::DiscardRandomOpponentActiveEnergy => {
            let opponent = (action.actor + 1) % 2;
            if let Some(energy) = state.get_active(opponent).attached_energy.last().copied() {
                state.discard_from_active(opponent, &[energy]);
            }
        }
        SimpleAction::ApplyStatusToOpponentActive { condition } => {
            let opponent = (action.actor + 1) % 2;
            state.apply_status_condition(opponent, 0, *condition);
        }
        SimpleAction::Noop => {}
        _ => panic!("Deterministic Action expected"),
    }
}

fn apply_attach_energy(
    state: &mut State,
    actor: usize,
    attachments: &[(u32, EnergyType, usize)],
    is_turn_energy: bool,
) {
    for (amount, energy, in_play_idx) in attachments {
        // it can happen that in the first iteration of for loop the pokemon was K.O.ed
        // if so, just skip the rest of the attachments.
        if state.in_play_pokemon[actor][*in_play_idx].is_none() {
            continue;
        }

        state.attach_energy_from_zone(actor, *in_play_idx, *energy, *amount, is_turn_energy);
    }
}

fn apply_attach_tool(state: &mut State, actor: usize, in_play_idx: usize, tool_card: &Card) {
    tools::ensure_tool_card(tool_card);
    let pokemon = state.in_play_pokemon[actor][in_play_idx]
        .as_mut()
        .expect("Pokemon should be there if attaching tool to it");
    pokemon.attached_tool = Some(tool_card.clone());

    // Steel Apron: "...recovers from all Special Conditions..." only for a [M] holder.
    if tools::has_tool(pokemon, crate::card_ids::CardId::A4153SteelApron)
        && pokemon.get_energy_type() == Some(crate::models::EnergyType::Metal)
    {
        pokemon.cure_status_conditions();
    }
}

fn apply_move_energy(
    state: &mut State,
    actor: usize,
    from_idx: usize,
    to_idx: usize,
    energy_type: EnergyType,
    amount: u32,
) {
    let actor_board = &mut state.in_play_pokemon[actor];
    let mut removed_energies = Vec::new();

    // Remove the specified amount of energy from source
    if let Some(from_card) = actor_board[from_idx].as_mut() {
        for _ in 0..amount {
            if let Some(pos) = from_card
                .attached_energy
                .iter()
                .position(|e| e == &energy_type)
            {
                from_card.attached_energy.swap_remove(pos);
                removed_energies.push(energy_type);
            } else {
                break; // No more energy of this type to remove
            }
        }
    }

    // Add removed energies to destination
    if !removed_energies.is_empty() {
        if let Some(to_card) = actor_board[to_idx].as_mut() {
            to_card.attached_energy.extend(removed_energies);
        } else if let Some(from_card) = actor_board[from_idx].as_mut() {
            // Put energies back if destination vanished (should not normally happen)
            from_card.attached_energy.extend(removed_energies);
        }
    }
}

pub(crate) fn apply_place_card(
    state: &mut State,
    actor: usize,
    card: &Card,
    index: usize,
    from_deck: bool,
) {
    let played_card = to_playable_card(card, true);
    state.in_play_pokemon[actor][index] = Some(played_card);
    state.refresh_starting_plains_bonus_for_idx(actor, index);
    // SoothingWind (Ogerpon ex) / Flower Shield (Comfey): cure status conditions on entry.
    if let Some(AbilityMechanic::SoothingWind { energy_type }) = get_ability_mechanic(card) {
        debug!("SoothingWind: Pokémon entered play – curing status conditions for player {actor}");
        state.apply_soothing_wind_for_player(actor, energy_type.as_ref());
    }
    if from_deck {
        state.remove_card_from_deck(actor, card);
    } else {
        state.remove_card_from_hand(actor, card);
        let placed_in_bench = index != 0;
        if placed_in_bench && has_ability_mechanic(card, &AbilityMechanic::InfiltratingInspection) {
            debug!("Misdreavus's Infiltrating Inspection: Opponent's hand is revealed (no-op in AI context)");
        }
        if placed_in_bench {
            on_bench_from_hand(actor, state, card, index);
        }
    }
}

fn apply_discard_fossil(acting_player: usize, state: &mut State, in_play_idx: usize) {
    // Discard the fossil from play (handles evolution chain and energies)
    state.discard_from_play(acting_player, in_play_idx);

    // If discarding from active spot, trigger promotion or declare winner
    if in_play_idx == 0 {
        state.trigger_promotion_or_declare_winner(acting_player);
    }
}

fn apply_return_pokemon_to_hand(acting_player: usize, state: &mut State, in_play_idx: usize) {
    let played_card = state.in_play_pokemon[acting_player][in_play_idx]
        .take()
        .expect("Pokemon should be there if returning to hand");
    let mut cards_to_collect = played_card.cards_behind.clone();
    cards_to_collect.push(played_card.card.clone());
    state.hands[acting_player].extend(cards_to_collect);

    // If returning the active, trigger promotion or declare winner.
    if in_play_idx == 0 {
        state.trigger_promotion_or_declare_winner(acting_player);
    }
}

fn apply_shuffle_in_play_pokemon_into_deck(
    acting_player: usize,
    state: &mut State,
    in_play_idx: usize,
) {
    let played_card = state.in_play_pokemon[acting_player][in_play_idx]
        .take()
        .expect("Pokemon should be there if shuffling into deck");
    let mut cards_to_shuffle = played_card.cards_behind.clone();
    cards_to_shuffle.push(played_card.card.clone());
    state.decks[acting_player].cards.extend(cards_to_shuffle);

    if in_play_idx == 0 {
        state.trigger_promotion_or_declare_winner(acting_player);
    }
}

fn apply_healing(
    acting_player: usize,
    state: &mut State,
    position: usize,
    amount: u32,
    cure_status: bool,
) {
    let pokemon = state.in_play_pokemon[acting_player][position]
        .as_mut()
        .expect("Pokemon should be there if healing it");
    pokemon.heal(amount);
    if cure_status {
        pokemon.cure_status_conditions();
    }
}

fn apply_schedule_delayed_spot_damage(
    state: &mut State,
    source_player: usize,
    target_player: usize,
    target_in_play_idx: usize,
    amount: u32,
) {
    state.add_turn_effect(
        TurnEffect::DelayedSpotDamage {
            source_player,
            target_player,
            target_in_play_idx,
            amount,
        },
        1,
    );
}

fn apply_heal_and_discard_energy(
    acting_player: usize,
    state: &mut State,
    position: usize,
    heal_amount: u32,
    discard_energies: &[EnergyType],
) {
    let pokemon = state.in_play_pokemon[acting_player][position]
        .as_mut()
        .expect("Pokemon should be there if healing it");
    let missing_hp = pokemon
        .get_effective_total_hp()
        .saturating_sub(pokemon.get_remaining_hp());
    let healed = heal_amount.min(missing_hp);
    pokemon.heal(heal_amount);

    if healed == 0 {
        return;
    }
    state.discard_energy_from_in_play(acting_player, position, discard_energies);
}

fn apply_move_all_damage(actor: usize, state: &mut State, from: usize, to: usize) {
    let damage_to_move = {
        let from_pokemon = state.in_play_pokemon[actor][from]
            .as_ref()
            .expect("Pokemon to move damage from should be there");
        from_pokemon.get_damage_counters()
    };

    if damage_to_move > 0 {
        let from_pokemon = state.in_play_pokemon[actor][from]
            .as_mut()
            .expect("Pokemon to move damage from should be there");
        from_pokemon.heal(damage_to_move);

        // Use handle_damage to ensure KO checks and other effects are triggered
        let targets = vec![(damage_to_move, actor, to)];
        // Attacking ref is (actor, from) as the source of the damage move
        handle_damage(state, (actor, from), &targets, false, None);
    }
}

/// is_free is analogous to "via retreat". If false, its because this comes from an Activate.
/// Note: This might be called when a K.O. happens, so can't assume there is an active...
fn apply_retreat(player: usize, state: &mut State, bench_idx: usize, is_free: bool) {
    if !is_free {
        let active = state.in_play_pokemon[player][0]
            .as_ref()
            .expect("Active Pokemon should be there if paid retreating");
        let double_grass = active.has_double_grass(state, player);
        let retreat_cost = get_retreat_cost(state, active).len();
        let attached_energy: &mut Vec<_> = state.in_play_pokemon[player][0]
            .as_mut()
            .expect("Active Pokemon should be there if paid retreating")
            .attached_energy
            .as_mut();

        // TODO: Maybe give option to user to select which energy to discard

        // Some energies are worth more than others... For now decide the ordering
        // that keeps as much Grass energy as possible (since possibly worth more).

        // Re-order energies so that Grass are at the beginning
        attached_energy.sort_by(|a, b| {
            if *a == EnergyType::Grass && *b != EnergyType::Grass {
                std::cmp::Ordering::Less
            } else if *a != EnergyType::Grass && *b == EnergyType::Grass {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        // Start walking from the back in the attached, removing energies until retreat cost is paid
        let mut remaining_cost = retreat_cost;
        let mut discarded: Vec<EnergyType> = vec![];
        while remaining_cost > 0 && !attached_energy.is_empty() {
            let energy = attached_energy.pop().unwrap();
            discarded.push(energy);
            if energy == EnergyType::Grass && double_grass {
                remaining_cost = remaining_cost.saturating_sub(2);
            } else {
                remaining_cost = remaining_cost.saturating_sub(1);
            }
        }
        if remaining_cost > 0 {
            panic!("Not enough energy to pay retreat cost");
        }

        if !discarded.is_empty() {
            state.discard_energies[player].extend(discarded);
        }

        state.has_retreated = true;
    }

    apply_activate(player, state, bench_idx);
}

// We will replace the PlayedCard, but taking into account the attached energy
//  and the remaining HP.
pub(crate) fn apply_evolve(
    acting_player: usize,
    state: &mut State,
    to_card: &Card,
    position: usize,
    from_deck: bool,
) {
    // This removes status conditions
    let mut played_card = to_playable_card(to_card, true);

    let from_pokemon = state.in_play_pokemon[acting_player][position]
        .as_ref()
        .expect("Pokemon should be there if evolving it");
    if let Card::Pokemon(to_pokemon) = &played_card.card {
        if to_pokemon.stage == 0 {
            panic!("Basic pokemon do not evolve from others...");
        }

        let damage_taken = from_pokemon.get_damage_counters();
        played_card.apply_damage(damage_taken);
        played_card.attached_energy = from_pokemon.attached_energy.clone();
        played_card.attached_tool = from_pokemon.attached_tool.clone();
        played_card.cards_behind = from_pokemon.cards_behind.clone();
        played_card.cards_behind.push(from_pokemon.card.clone());
        state.in_play_pokemon[acting_player][position] = Some(played_card);
        state.refresh_starting_plains_bonus_for_idx(acting_player, position);
    } else {
        panic!("Only Pokemon cards can be evolved");
    }

    // Remove the evolution card from either hand or deck depending on the source
    if from_deck {
        state.remove_card_from_deck(acting_player, to_card);
    } else {
        state.remove_card_from_hand(acting_player, to_card);
    }

    // Run special logic hooks on evolution
    on_evolve(acting_player, state, to_card, !from_deck)
}

fn forecast_pokemon_communication(
    acting_player: usize,
    state: &State,
    hand_pokemon: &Card,
) -> Outcomes {
    let deck_pokemon: Vec<_> = state.iter_deck_pokemon(acting_player).collect();

    let num_deck_pokemon = deck_pokemon.len();
    if num_deck_pokemon == 0 {
        // Should not happen if move generation is correct, but just shuffle deck
        return Outcomes::single_fn(|rng, state, action| {
            state.decks[action.actor].shuffle(false, rng);
        });
    }

    // Create uniform probability for each deck Pokemon (1/N for each)
    let probabilities = vec![1.0 / (num_deck_pokemon as f64); num_deck_pokemon];
    let mut outcomes: Mutations = vec![];
    for i in 0..num_deck_pokemon {
        let hand_pokemon_clone = hand_pokemon.clone();
        outcomes.push(Box::new(move |rng, state, action| {
            // Get the i-th Pokemon from deck
            let deck_pokemon_card = state
                .iter_deck_pokemon(action.actor)
                .nth(i)
                .cloned()
                .expect("Deck Pokemon should exist");

            // Perform the swap
            // 1. Transfer hand Pokemon to deck
            state.transfer_card_from_hand_to_deck(action.actor, &hand_pokemon_clone);
            // 2. Transfer deck Pokemon to hand
            state.transfer_card_from_deck_to_hand(action.actor, &deck_pokemon_card);
            // 5. Shuffle deck
            state.decks[action.actor].shuffle(false, rng);

            debug!(
                "Pokemon Communication: Swapped {:?} from hand with {:?} from deck",
                hand_pokemon_clone, deck_pokemon_card
            );
        }));
    }

    Outcomes::from_parts(probabilities, outcomes)
}

fn forecast_shuffle_pokemon_into_deck(acting_player: usize, hand_pokemon: &[Card]) -> Outcomes {
    let pokemon_list = hand_pokemon.to_vec();
    Outcomes::single_fn(move |rng, state, _action| {
        for pokemon in &pokemon_list {
            state.transfer_card_from_hand_to_deck(acting_player, pokemon);
        }
        state.decks[acting_player].shuffle(false, rng);
        debug!("May: Shuffled {:?} from hand into deck", pokemon_list);
    })
}

fn forecast_shuffle_own_cards_into_deck(acting_player: usize, cards: &[Card]) -> Outcomes {
    let cards_to_shuffle = cards.to_vec();
    Outcomes::single_fn(move |rng, state, _action| {
        for card in &cards_to_shuffle {
            state.transfer_card_from_hand_to_deck(acting_player, card);
        }
        state.decks[acting_player].shuffle(false, rng);
        state.maybe_draw_card(acting_player);
        debug!(
            "Maintenance: Shuffled {:?} from hand into deck, then drew a card",
            cards_to_shuffle
        );
    })
}

fn forecast_shuffle_opponent_supporter(acting_player: usize, supporter_card: &Card) -> Outcomes {
    let supporter_clone = supporter_card.clone();
    Outcomes::single_fn(move |rng, state, _action| {
        let opponent = (acting_player + 1) % 2;
        state.transfer_card_from_hand_to_deck(opponent, &supporter_clone);
        state.decks[opponent].shuffle(false, rng);
        debug!(
            "Silver: Shuffled {:?} from opponent's hand into their deck",
            supporter_clone
        );
    })
}

fn forecast_discard_opponent_supporter(acting_player: usize, supporter_card: &Card) -> Outcomes {
    let supporter_clone = supporter_card.clone();
    Outcomes::single_fn(move |_rng, state, _action| {
        let opponent = (acting_player + 1) % 2;
        state.discard_card_from_hand(opponent, &supporter_clone);
        debug!(
            "Mega Absol Ex: Discarded {:?} from opponent's hand",
            supporter_clone
        );
    })
}

fn forecast_discard_own_cards(acting_player: usize, cards: &[Card]) -> Outcomes {
    let cards_clone = cards.to_vec();
    Outcomes::single_fn(move |_rng, state, _action| {
        for card in &cards_clone {
            state.discard_card_from_hand(acting_player, card);
        }
        debug!("Discarded {:?} from hand", cards_clone);
    })
}

fn forecast_attach_from_discard(
    state: &State,
    acting_player: usize,
    in_play_idx: usize,
    num_random_energies: usize,
) -> Outcomes {
    let discard_energies = &state.discard_energies[acting_player];
    let actual_num = std::cmp::min(num_random_energies, discard_energies.len());

    if actual_num == 0 {
        return Outcomes::single_fn(|_, _, _| {});
    }
    if actual_num == 1 {
        // Deterministic: just attach the first energy
        let energy = discard_energies[0];
        return Outcomes::single_fn(move |_rng, state, action| {
            state.attach_energy_from_discard(action.actor, in_play_idx, &[energy]);
            debug!(
                "Lusamine: Attached {:?} from discard to Pokemon at index {}",
                energy, in_play_idx
            );
        });
    }

    // For 2 energies, generate all combinations and deduplicate
    let combinations = generate_energy_combinations(discard_energies);
    let total_combinations: usize = combinations.iter().map(|(_, count)| count).sum();

    let mut probabilities = Vec::new();
    let mut mutations: Mutations = Vec::new();
    for (combo, count) in combinations {
        let probability = count as f64 / total_combinations as f64;
        probabilities.push(probability);
        mutations.push(Box::new(move |_rng, state, action| {
            state.attach_energy_from_discard(action.actor, in_play_idx, &combo);
            debug!(
                "Lusamine: Attached {:?} from discard to Pokemon at index {}",
                combo, in_play_idx
            );
        }));
    }

    Outcomes::from_parts(probabilities, mutations)
}

/// Volkner: deterministically attach up to `count` energies of `energy_type` from discard.
fn forecast_attach_typed_from_discard(
    in_play_idx: usize,
    energy_type: EnergyType,
    count: usize,
) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let available = state.discard_energies[action.actor]
            .iter()
            .filter(|e| **e == energy_type)
            .count();
        let energies = vec![energy_type; std::cmp::min(count, available)];
        state.attach_energy_from_discard(action.actor, in_play_idx, &energies);
    })
}

/// Generate all unique 2-energy combinations from a list of energies in discard pile.
/// Returns a vector of (combination, count) tuples where count is how many times
/// this combination appears when considering all possible pairs.
fn generate_energy_combinations(energies: &[EnergyType]) -> Vec<(Vec<EnergyType>, usize)> {
    let mut combination_counts: HashMap<Vec<EnergyType>, usize> = HashMap::new();
    for i in 0..energies.len() {
        for j in (i + 1)..energies.len() {
            let mut combo = vec![energies[i], energies[j]];
            combo.sort(); // Sort to treat [Grass, Fire] same as [Fire, Grass]
            *combination_counts.entry(combo).or_insert(0) += 1;
        }
    }
    combination_counts.into_iter().collect()
}

fn forecast_sada_attach(assignments: &[(crate::models::EnergyType, usize)]) -> Outcomes {
    let assignments = assignments.to_vec();
    Outcomes::single_fn(move |_, state, action| {
        for (energy, in_play_idx) in &assignments {
            state.attach_energy_from_discard(action.actor, *in_play_idx, &[*energy]);
        }
    })
}

fn apply_eevee_bag_damage_boost(state: &mut State) {
    use crate::effects::TurnEffect;
    state.add_turn_effect(
        TurnEffect::IncreasedDamageForEeveeEvolutions { amount: 10 },
        0,
    );
}

fn apply_heal_all_eevee_evolutions(acting_player: usize, state: &mut State) {
    for pokemon in state.in_play_pokemon[acting_player].iter_mut().flatten() {
        if pokemon.evolved_from("Eevee") {
            pokemon.heal(20);
        }
    }
}

// Test that when evolving a damanged pokemon, damage stays.
#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;
    use crate::card_ids::CardId;
    use crate::database::get_card_by_enum;
    use crate::{
        models::{EnergyType, PlayedCard},
        Deck,
    };

    #[test]
    fn test_apply_evolve() {
        let mut state = State::new(&Deck::default(), &Deck::default());
        let energy = EnergyType::Colorless;
        let mankey = get_card_by_enum(CardId::PA017Mankey);
        let primeape = get_card_by_enum(CardId::A1142Primeape);
        let mut base_played_card = to_playable_card(&mankey, false);
        base_played_card.apply_damage(30); // 30 damage taken
        base_played_card.attached_energy = vec![energy];
        state.in_play_pokemon[0][0] = Some(base_played_card.clone());
        let mut healthy_bench = base_played_card.clone();
        healthy_bench.heal(30);
        healthy_bench.attached_energy = vec![energy, energy, energy];
        state.in_play_pokemon[0][2] = Some(healthy_bench);
        state.hands[0] = vec![primeape.clone(), primeape.clone()];

        // Evolve Active
        apply_evolve(0, &mut state, &primeape, 0, false);
        assert_eq!(
            state.in_play_pokemon[0][0],
            Some(PlayedCard::new(
                primeape.clone(),
                30, // 30 damage counters
                90,
                vec![energy],
                true,
                vec![mankey.clone()]
            ))
        );

        // Evolve Bench
        apply_evolve(0, &mut state, &primeape, 2, false);
        assert_eq!(
            state.in_play_pokemon[0][0],
            Some(PlayedCard::new(
                primeape.clone(),
                30, // 30 damage counters
                90,
                vec![energy],
                true,
                vec![mankey.clone()]
            ))
        );
        assert_eq!(
            state.in_play_pokemon[0][2],
            Some(PlayedCard::new(
                primeape.clone(),
                0, // 0 damage counters
                90,
                vec![energy, energy, energy],
                true,
                vec![mankey.clone()]
            ))
        );
    }

    #[test]
    fn test_forcefully_retreat() {
        let mut state = State::new(&Deck::default(), &Deck::default());
        // PUT Mankey in Active and Primeape in Bench 2
        let mankey = get_card_by_enum(CardId::A1141Mankey);
        let primeape = get_card_by_enum(CardId::A1142Primeape);
        state.in_play_pokemon[0][0] = Some(to_playable_card(&mankey, false));
        state.in_play_pokemon[0][2] = Some(to_playable_card(&primeape, false));

        // Forcefully Activate Primeape
        let mut rng: StdRng = StdRng::seed_from_u64(rand::random());
        let action = Action {
            actor: 0,
            action: SimpleAction::Activate {
                player: 0,
                in_play_idx: 2,
            },
            is_stack: false,
        };
        apply_action(&mut rng, &mut state, &action);

        let mut expected_primeape = to_playable_card(&primeape, false);
        expected_primeape.moved_to_active_this_turn = true;
        assert_eq!(state.in_play_pokemon[0][0], Some(expected_primeape));
        assert_eq!(
            state.in_play_pokemon[0][2],
            Some(to_playable_card(&mankey, false))
        );
    }

    #[test]
    fn test_generate_energy_combinations_all_same_type() {
        // [Grass, Grass, Grass] -> 1 unique combo [Grass, Grass] with count 3
        let energies = vec![EnergyType::Grass, EnergyType::Grass, EnergyType::Grass];
        let combinations = super::generate_energy_combinations(&energies);

        assert_eq!(combinations.len(), 1);
        let (combo, count) = &combinations[0];
        assert_eq!(combo, &vec![EnergyType::Grass, EnergyType::Grass]);
        assert_eq!(*count, 3);
    }

    #[test]
    fn test_generate_energy_combinations_mixed_types() {
        // [Grass, Grass, Fire] -> 2 unique combos:
        // [Grass, Grass] count 1, [Fire, Grass] or [Grass, Fire] count 2
        let energies = vec![EnergyType::Grass, EnergyType::Grass, EnergyType::Fire];
        let combinations = super::generate_energy_combinations(&energies);

        assert_eq!(combinations.len(), 2);

        // Find the mixed Fire-Grass combo (sorted, so could be either order)
        let fire_grass = combinations
            .iter()
            .find(|(combo, _)| {
                combo.len() == 2
                    && combo.contains(&EnergyType::Fire)
                    && combo.contains(&EnergyType::Grass)
                    && combo[0] != combo[1]
            })
            .expect("Should have Fire-Grass combo");
        assert_eq!(fire_grass.1, 2);

        let grass_grass = combinations
            .iter()
            .find(|(combo, _)| combo == &vec![EnergyType::Grass, EnergyType::Grass])
            .expect("Should have Grass-Grass combo");
        assert_eq!(grass_grass.1, 1);
    }

    #[test]
    fn test_generate_energy_combinations_all_different() {
        // [Grass, Fire, Water] -> 3 unique combos, each count 1
        let energies = vec![EnergyType::Grass, EnergyType::Fire, EnergyType::Water];
        let combinations = super::generate_energy_combinations(&energies);

        assert_eq!(combinations.len(), 3); // C(3,2) = 3
        for (_, count) in &combinations {
            assert_eq!(*count, 1);
        }
    }
}
