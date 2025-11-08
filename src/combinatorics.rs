/// Generate all k-combinations from a slice
///
/// Returns all possible ways to choose k items from the input slice,
/// where order doesn't matter.
///
/// # Examples
///
/// ```
/// use deckgym::combinatorics::generate_combinations;
///
/// let items = vec![1, 2, 3];
/// let combos = generate_combinations(&items, 2);
/// assert_eq!(combos.len(), 3); // C(3,2) = 3
/// // Returns: [[1, 2], [1, 3], [2, 3]]
/// ```
pub fn generate_combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    if items.is_empty() {
        return vec![];
    }

    let mut result = Vec::new();

    // Include the first item
    let with_first = generate_combinations(&items[1..], k - 1);
    for mut combo in with_first {
        combo.insert(0, items[0].clone());
        result.push(combo);
    }

    // Exclude the first item
    let without_first = generate_combinations(&items[1..], k);
    result.extend(without_first);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_combinations_empty() {
        let items: Vec<i32> = vec![];
        let combos = generate_combinations(&items, 2);
        assert_eq!(combos.len(), 0);
    }

    #[test]
    fn test_generate_combinations_k_zero() {
        let items = vec![1, 2, 3];
        let combos = generate_combinations(&items, 0);
        assert_eq!(combos.len(), 1);
        assert_eq!(combos[0].len(), 0);
    }

    #[test]
    fn test_generate_combinations_2_from_3() {
        let items = vec![1, 2, 3];
        let combos = generate_combinations(&items, 2);
        assert_eq!(combos.len(), 3); // C(3,2) = 3
        assert!(combos.contains(&vec![1, 2]));
        assert!(combos.contains(&vec![1, 3]));
        assert!(combos.contains(&vec![2, 3]));
    }

    #[test]
    fn test_generate_combinations_3_from_4() {
        let items = vec!['a', 'b', 'c', 'd'];
        let combos = generate_combinations(&items, 3);
        assert_eq!(combos.len(), 4); // C(4,3) = 4
        assert!(combos.contains(&vec!['a', 'b', 'c']));
        assert!(combos.contains(&vec!['a', 'b', 'd']));
        assert!(combos.contains(&vec!['a', 'c', 'd']));
        assert!(combos.contains(&vec!['b', 'c', 'd']));
    }

    #[test]
    fn test_generate_combinations_all_items() {
        let items = vec![1, 2, 3];
        let combos = generate_combinations(&items, 3);
        assert_eq!(combos.len(), 1); // C(3,3) = 1
        assert_eq!(combos[0], vec![1, 2, 3]);
    }
}
