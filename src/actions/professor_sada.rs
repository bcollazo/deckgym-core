use crate::combinatorics::generate_combinations;
use crate::models::EnergyType;
use std::collections::HashSet;

/// Generate all valid ways to attach up to 3 different energy types from the discard pile
/// to Ancient Pokémon in play.
///
/// Attaches `min(3, distinct_types_in_discard)` energies — one of each chosen type.
/// If there are more than 3 distinct types the player also chooses which 3 to use.
///
/// `ancient_slots` are the in-play indices that hold Ancient Pokémon.
/// `discard_energies` is the full discard-energy list for the player (duplicates allowed).
///
/// Returns every possible complete assignment as a vec of `(EnergyType, in_play_idx)` pairs.
pub fn generate_professor_sada_assignments(
    ancient_slots: &[usize],
    discard_energies: &[EnergyType],
) -> Vec<Vec<(EnergyType, usize)>> {
    if ancient_slots.is_empty() || discard_energies.is_empty() {
        return vec![];
    }

    // Collect unique energy types, preserving first-seen order
    let mut seen = HashSet::new();
    let unique_types: Vec<EnergyType> = discard_energies
        .iter()
        .filter(|e| seen.insert(*e))
        .copied()
        .collect();

    let n_to_attach = unique_types.len().min(3);
    let mut results = Vec::new();
    for types in generate_combinations(&unique_types, n_to_attach) {
        results.extend(all_slot_assignments(&types, ancient_slots));
    }
    results
}

/// Generate every way to assign each energy type (in order) to any ancient slot.
/// This is the Cartesian product of `ancient_slots` taken `types.len()` times, zipped with types.
fn all_slot_assignments(
    types: &[EnergyType],
    ancient_slots: &[usize],
) -> Vec<Vec<(EnergyType, usize)>> {
    if types.is_empty() {
        return vec![vec![]];
    }
    let tails = all_slot_assignments(&types[1..], ancient_slots);
    let mut result = Vec::new();
    for &slot in ancient_slots {
        for tail in &tails {
            let mut assignment = vec![(types[0], slot)];
            assignment.extend_from_slice(tail);
            result.push(assignment);
        }
    }
    result
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
    fn test_empty_discard() {
        let result = generate_professor_sada_assignments(&[0, 1], &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_one_type_two_ancient_slots() {
        // 1 distinct type → attach 1 energy, player picks which slot
        let result =
            generate_professor_sada_assignments(&[0, 1], &[EnergyType::Fire, EnergyType::Fire]);
        // C(1,1) × 2^1 = 2 assignments
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|a| a.len() == 1));
        assert!(result.iter().all(|a| a[0].0 == EnergyType::Fire));
    }

    #[test]
    fn test_two_types_two_ancient_slots() {
        // 2 distinct types → attach 2 energies (one of each)
        let result =
            generate_professor_sada_assignments(&[0, 1], &[EnergyType::Fire, EnergyType::Water]);
        // C(2,2) × 2^2 = 1 × 4 = 4 assignments
        assert_eq!(result.len(), 4);
        assert!(result.iter().all(|a| a.len() == 2));
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
        // Duplicates should not inflate the type count
        let result = generate_professor_sada_assignments(
            &[0, 1],
            &[
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Water,
                EnergyType::Grass,
            ],
        );
        // 3 distinct types: 1 combo × 2^3 = 8
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_each_assignment_uses_distinct_types() {
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
            assert_eq!(
                types.len(),
                assignment.len(),
                "Each assignment must use distinct types"
            );
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
