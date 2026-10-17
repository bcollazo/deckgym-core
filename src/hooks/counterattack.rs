use crate::{
    actions::abilities::AbilityMechanic, card_ids::CardId, effects::CardEffect, models::PlayedCard,
    tools::has_tool,
};

/// Some cards counterattack either because of RockyHelmet or because of their own ability.
pub(crate) fn get_counterattack_damage(card: &PlayedCard) -> u32 {
    let mut total_damage = 0;
    if has_tool(card, CardId::A2148RockyHelmet) {
        total_damage += 20;
    }

    // Temporary counterattack effects (e.g. Alolan Sandslash's Spike Armor).
    total_damage += card
        .get_active_effects()
        .iter()
        .filter_map(|effect| match effect {
            CardEffect::Counterattack { amount } => Some(*amount),
            _ => None,
        })
        .sum::<u32>();

    // Some cards have it as an ability (Rough Skin and its reprints, Iron Jugulis).
    if let Some(AbilityMechanic::CounterattackDamage { amount }) = card.ability_mechanic() {
        total_damage += amount;
    }

    total_damage
}

/// Check if the defending Pokemon should poison the attacker when damaged.
/// Returns true if the attacker should be poisoned.
pub(crate) fn should_poison_attacker(card: &PlayedCard) -> bool {
    if has_tool(card, CardId::A3146PoisonBarb) {
        return true;
    }

    // Some cards have it as an ability (Dragalge ex's Poison Point)
    if card.has_ability(&AbilityMechanic::PoisonAttackerOnDamaged) {
        return true;
    }

    false
}
