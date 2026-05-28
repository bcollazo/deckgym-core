use crate::combinatorics::generate_combinations;
use crate::models::EnergyType;
use std::collections::HashSet;

/// Generate all valid ways to attach 3 different energy types from the discard pile
/// to Ancient Pokémon in play.
///
/// `ancient_slots` are the in-play indices that hold Ancient Pokémon.
/// `discard_energies` is the full discard-energy list for the player (duplicates allowed).
///
/// Returns every possible complete assignment as a vec of 3 `(EnergyType, in_play_idx)` pairs
/// representing which energy goes to which Ancient Pokémon slot.
pub fn generate_professor_sada_assignments(
    ancient_slots: &[usize],
    discard_energies: &[EnergyType],
) -> Vec<Vec<(EnergyType, usize)>> {
    if ancient_slots.is_empty() {
        return vec![];
    }

    // Collect unique energy types, preserving first-seen order
    let mut seen = HashSet::new();
    let unique_types: Vec<EnergyType> = discard_energies
        .iter()
        .filter(|e| seen.insert(*e))
        .copied()
        .collect();

    if unique_types.len() < 3 {
        return vec![];
    }

    // For each 3-combination of distinct types, generate every possible target assignment
    let mut results = Vec::new();
    for types in generate_combinations(&unique_types, 3) {
        for &slot0 in ancient_slots {
            for &slot1 in ancient_slots {
                for &slot2 in ancient_slots {
                    results.push(vec![
                        (types[0], slot0),
                        (types[1], slot1),
                        (types[2], slot2),
                    ]);
                }
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::EnergyType;

    #[test]
    fn test_no_ancient_pokemon() {
        let result = generate_professor_sada_assignments(
            &[],
            &[EnergyType::Fire, EnergyType::Water, EnergyType::Grass],
        );
        assert!(result.is_empty());
    }

    #[test]
    fn test_fewer_than_3_types_in_discard() {
        let result =
            generate_professor_sada_assignments(&[0, 1], &[EnergyType::Fire, EnergyType::Water]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_type_in_discard() {
        let result =
            generate_professor_sada_assignments(&[0, 1], &[EnergyType::Fire, EnergyType::Fire]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_exactly_3_types_one_ancient_slot() {
        let result = generate_professor_sada_assignments(
            &[0],
            &[EnergyType::Fire, EnergyType::Water, EnergyType::Grass],
        );
        // 1 type combo × 1^3 = 1 assignment (all energies go to the only slot)
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            vec![
                (EnergyType::Fire, 0),
                (EnergyType::Water, 0),
                (EnergyType::Grass, 0),
            ]
        );
    }

    #[test]
    fn test_exactly_3_types_two_ancient_slots() {
        let result = generate_professor_sada_assignments(
            &[0, 1],
            &[EnergyType::Fire, EnergyType::Water, EnergyType::Grass],
        );
        // 1 type combo × 2^3 = 8 assignments
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_four_types_two_ancient_slots() {
        let result = generate_professor_sada_assignments(
            &[0, 1],
            &[
                EnergyType::Fire,
                EnergyType::Water,
                EnergyType::Grass,
                EnergyType::Lightning,
            ],
        );
        // C(4,3) × 2^3 = 4 × 8 = 32 assignments
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_three_types_four_ancient_slots() {
        let result = generate_professor_sada_assignments(
            &[0, 1, 2, 3],
            &[EnergyType::Fire, EnergyType::Water, EnergyType::Grass],
        );
        // 1 type combo × 4^3 = 64 assignments
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_duplicate_energies_counted_as_one_type() {
        // Duplicates in discard should not inflate the type count
        let result = generate_professor_sada_assignments(
            &[0, 1],
            &[
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Water,
                EnergyType::Grass,
            ],
        );
        // Same as 3 distinct types: 1 combo × 2^3 = 8
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_all_assignments_have_exactly_3_entries() {
        let result = generate_professor_sada_assignments(
            &[0, 1],
            &[EnergyType::Fire, EnergyType::Water, EnergyType::Grass],
        );
        for assignment in &result {
            assert_eq!(assignment.len(), 3);
        }
    }

    #[test]
    fn test_each_assignment_uses_three_distinct_types() {
        let result = generate_professor_sada_assignments(
            &[0, 1],
            &[
                EnergyType::Fire,
                EnergyType::Water,
                EnergyType::Grass,
                EnergyType::Lightning,
            ],
        );
        for assignment in &result {
            let types: HashSet<EnergyType> = assignment.iter().map(|(e, _)| *e).collect();
            assert_eq!(types.len(), 3, "Each assignment must use 3 distinct types");
        }
    }

    #[test]
    fn test_all_slots_are_valid_ancient_slots() {
        let ancient_slots = &[0_usize, 2];
        let result = generate_professor_sada_assignments(
            ancient_slots,
            &[EnergyType::Fire, EnergyType::Water, EnergyType::Grass],
        );
        for assignment in &result {
            for (_, slot) in assignment {
                assert!(
                    ancient_slots.contains(slot),
                    "Slot {slot} is not a valid ancient slot"
                );
            }
        }
    }
}
