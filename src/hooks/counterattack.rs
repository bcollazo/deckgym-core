use crate::{card_ids::CardId, models::PlayedCard, tool_ids::ToolId};

/// Some cards counterattack either because of RockyHelmet or because of their own ability.
pub(crate) fn get_counterattack_damage(card: &PlayedCard) -> u32 {
    let mut total_damage = 0;
    if let Some(tool) = card.attached_tool {
        if tool == ToolId::A2148RockyHelmet {
            total_damage += 20;
        }
    }

    // Some cards have it as an ability
    let card_id = CardId::from_card_id(&card.card.get_id());
    match card_id {
        Some(CardId::A1061Poliwrath)
        | Some(CardId::A1a056Druddigon)
        | Some(CardId::A2b028Pawmot)
        | Some(CardId::A3a052Ferrothorn)
        | Some(CardId::A4a065Zangoose)
        | Some(CardId::B1297Poliwrath)
        | Some(CardId::PA054Pawmot) => {
            total_damage += 20;
        }
        _ => {}
    }

    total_damage
}

/// Check if the defending Pokemon should poison the attacker when damaged.
/// Returns true if the attacker should be poisoned.
pub(crate) fn should_poison_attacker(card: &PlayedCard) -> bool {
    if let Some(tool) = card.attached_tool {
        if tool == ToolId::A3146PoisonBarb {
            return true;
        }
    }

    // Some cards have it as an ability (Dragalge ex's Poison Point)
    let card_id = CardId::from_card_id(&card.card.get_id());
    match card_id {
        Some(CardId::B1160DragalgeEx)
        | Some(CardId::B1263DragalgeEx)
        | Some(CardId::B1281DragalgeEx) => {
            return true;
        }
        _ => {}
    }

    false
}
