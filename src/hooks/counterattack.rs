use crate::{card_ids::CardId, tool_ids::ToolId, types::PlayedCard};

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
        Some(CardId::A1061Poliwrath) | Some(CardId::A1a056Druddigon) => {
            total_damage += 20;
        }
        _ => {}
    }

    total_damage
}
