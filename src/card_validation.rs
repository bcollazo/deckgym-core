use crate::{
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, TrainerType},
    move_generation::generate_possible_trainer_actions,
    state::State,
    AbilityId, AttackId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImplementationStatus {
    Complete,
    CardNotFound,
    MissingAttack,
    MissingAbility,
    MissingTrainer,
    MissingTool,
}

impl ImplementationStatus {
    pub fn is_complete(&self) -> bool {
        matches!(self, ImplementationStatus::Complete)
    }

    pub fn description(&self) -> &'static str {
        match self {
            ImplementationStatus::Complete => "Fully implemented",
            ImplementationStatus::CardNotFound => "Card ID not found",
            ImplementationStatus::MissingAttack => "Attack effect not implemented",
            ImplementationStatus::MissingAbility => "Ability not implemented",
            ImplementationStatus::MissingTrainer => "Trainer logic not implemented",
            ImplementationStatus::MissingTool => "Tool not implemented",
        }
    }
}

pub fn get_implementation_status(card_id: CardId) -> ImplementationStatus {
    let card = get_card_by_enum(card_id);
    let card_id_string = card.get_id();

    match card {
        Card::Pokemon(pokemon) => {
            // Verify attacks have no effects or effects are implemented
            for (index, attack) in pokemon.attacks.iter().enumerate() {
                if attack.effect.is_some()
                    && AttackId::from_pokemon_index(&card_id_string, index).is_none()
                {
                    return ImplementationStatus::MissingAttack;
                }
            }

            // Verify ability is implemented
            if pokemon.ability.is_some() && AbilityId::from_pokemon_id(&card_id_string).is_none() {
                return ImplementationStatus::MissingAbility;
            }
        }
        Card::Trainer(trainer_card) => {
            if trainer_card.trainer_card_type == TrainerType::Tool
                && crate::tool_ids::ToolId::from_trainer_card(&trainer_card).is_none()
            {
                return ImplementationStatus::MissingTool;
            }

            // Verify it can generate moves
            let moves = generate_possible_trainer_actions(&State::default(), &trainer_card);
            if moves.is_none() {
                return ImplementationStatus::MissingTrainer;
            };
        }
    }

    ImplementationStatus::Complete
}
