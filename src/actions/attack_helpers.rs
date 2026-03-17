use crate::{actions::SimpleAction, models::EnergyType, State};

pub(crate) fn collect_in_play_indices_by_type(
    state: &State,
    player: usize,
    energy_type: EnergyType,
) -> Vec<usize> {
    state
        .enumerate_in_play_pokemon(player)
        .filter_map(|(in_play_idx, pokemon)| {
            if pokemon.get_energy_type() == Some(energy_type) {
                Some(in_play_idx)
            } else {
                None
            }
        })
        .collect()
}

pub(crate) fn energy_any_way_choices(
    target_indices: &[usize],
    energy_type: EnergyType,
    count: usize,
) -> Vec<SimpleAction> {
    if target_indices.is_empty() || count == 0 {
        return Vec::new();
    }

    let mut distributions = Vec::new();
    generate_distributions(
        target_indices,
        count,
        0,
        &mut vec![0; target_indices.len()],
        &mut distributions,
    );

    let mut choices = Vec::new();
    for dist in distributions {
        let mut attachments = Vec::new();
        for (i, &pokemon_idx) in target_indices.iter().enumerate() {
            if dist[i] > 0 {
                attachments.push((dist[i] as u32, energy_type, pokemon_idx));
            }
        }
        if !attachments.is_empty() {
            choices.push(SimpleAction::Attach {
                attachments,
                is_turn_energy: false,
            });
        }
    }

    choices
}

// Helper function to generate all possible distributions of 'count' energy
// across the available Pokémon
pub(crate) fn generate_distributions(
    target_indices: &[usize],
    remaining: usize,
    start_idx: usize,
    current: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if remaining == 0 {
        result.push(current.clone());
        return;
    }

    if start_idx >= target_indices.len() {
        return;
    }

    // Try different amounts for the current Pokémon
    for amount in 0..=remaining {
        current[start_idx] = amount;
        generate_distributions(
            target_indices,
            remaining - amount,
            start_idx + 1,
            current,
            result,
        );
    }
    current[start_idx] = 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_energy_any_way_choices_empty_or_zero() {
        let choices = energy_any_way_choices(&[], EnergyType::Psychic, 3);
        assert!(choices.is_empty());

        let choices = energy_any_way_choices(&[0, 2], EnergyType::Psychic, 0);
        assert!(choices.is_empty());
    }

    #[test]
    fn test_energy_any_way_choices_two_targets_three_energy() {
        let choices = energy_any_way_choices(&[0, 2], EnergyType::Psychic, 3);
        assert_eq!(choices.len(), 4);

        let expected = [
            vec![(3, EnergyType::Psychic, 2)],
            vec![(1, EnergyType::Psychic, 0), (2, EnergyType::Psychic, 2)],
            vec![(2, EnergyType::Psychic, 0), (1, EnergyType::Psychic, 2)],
            vec![(3, EnergyType::Psychic, 0)],
        ];

        for (i, choice) in choices.iter().enumerate() {
            if let SimpleAction::Attach { attachments, .. } = choice {
                assert_eq!(attachments, &expected[i]);
            } else {
                panic!("Expected SimpleAction::Attach");
            }
        }
    }

    #[test]
    fn test_energy_any_way_choices_three_targets_two_energy() {
        let choices = energy_any_way_choices(&[0, 1, 3], EnergyType::Psychic, 2);
        assert_eq!(choices.len(), 6);

        let expected = [
            vec![(2, EnergyType::Psychic, 3)],
            vec![(1, EnergyType::Psychic, 1), (1, EnergyType::Psychic, 3)],
            vec![(2, EnergyType::Psychic, 1)],
            vec![(1, EnergyType::Psychic, 0), (1, EnergyType::Psychic, 3)],
            vec![(1, EnergyType::Psychic, 0), (1, EnergyType::Psychic, 1)],
            vec![(2, EnergyType::Psychic, 0)],
        ];

        for (i, choice) in choices.iter().enumerate() {
            if let SimpleAction::Attach { attachments, .. } = choice {
                assert_eq!(attachments, &expected[i]);
            } else {
                panic!("Expected SimpleAction::Attach");
            }
        }
    }
}
