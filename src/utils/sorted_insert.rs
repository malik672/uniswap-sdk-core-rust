use crate::prelude::*;

/// Given an array of items sorted by `comparator`, insert an item into its sort index and constrain the size to
/// `maxSize` by removing the last item
pub fn sorted_insert<T: Clone>(
    items: &mut Vec<T>,
    add: T,
    max_size: usize,
    comparator: fn(&T, &T) -> Ordering,
) -> Option<T> {
    assert!(max_size > 0, "MAX_SIZE_ZERO");
    assert!(items.len() <= max_size, "ITEMS_SIZE");

    let removed_item = if items.len() == max_size {
        match items.last() {
            Some(last) if comparator(&add, last) != Ordering::Greater => items.pop(),
            // short circuit if full and the additional item does not come before the last item
            _ => return Some(add),
        }
    } else {
        None
    };

    let pos = match items.binary_search_by(|i| comparator(i, &add)) {
        Ok(pos) | Err(pos) => pos,
    };

    items.insert(pos, add);
    removed_item
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmp(a: &i32, b: &i32) -> Ordering {
        a.cmp(b)
    }

    fn reverse_cmp(a: &i32, b: &i32) -> Ordering {
        b.cmp(a)
    }

    #[test]
    #[should_panic(expected = "MAX_SIZE_ZERO")]
    fn test_max_size_zero() {
        let mut arr = Vec::new();
        sorted_insert(&mut arr, 1, 0, cmp);
    }

    #[test]
    #[should_panic(expected = "ITEMS_SIZE")]
    fn test_length_greater_than_max_size() {
        let mut arr = vec![1, 2];
        sorted_insert(&mut arr, 1, 1, cmp);
    }

    #[test]
    fn test_add_if_empty() {
        let mut arr = Vec::new();
        assert_eq!(sorted_insert(&mut arr, 3, 2, cmp), None);
        assert_eq!(arr, vec![3]);
    }

    #[test]
    fn test_add_if_not_full() {
        let mut arr = vec![1, 5];
        assert_eq!(sorted_insert(&mut arr, 3, 3, cmp), None);
        assert_eq!(arr, vec![1, 3, 5]);
    }

    #[test]
    fn test_add_if_will_not_be_full_after() {
        let mut arr = vec![1];
        assert_eq!(sorted_insert(&mut arr, 0, 3, cmp), None);
        assert_eq!(arr, vec![0, 1]);
    }

    #[test]
    fn test_return_add_if_sorts_after_last() {
        let mut arr = vec![1, 2, 3];
        assert_eq!(sorted_insert(&mut arr, 4, 3, cmp), Some(4));
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    fn test_remove_from_end_if_full() {
        let mut arr = vec![1, 3, 4];
        assert_eq!(sorted_insert(&mut arr, 2, 3, cmp), Some(4));
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    fn test_uses_comparator() {
        let mut arr = vec![4, 2, 1];
        assert_eq!(sorted_insert(&mut arr, 3, 3, reverse_cmp), Some(1));
        assert_eq!(arr, vec![4, 3, 2]);
    }

    #[test]
    fn test_max_size_of_1_empty_add() {
        let mut arr = Vec::new();
        assert_eq!(sorted_insert(&mut arr, 3, 1, cmp), None);
        assert_eq!(arr, vec![3]);
    }

    #[test]
    fn test_max_size_of_1_full_add_greater() {
        let mut arr = vec![2];
        assert_eq!(sorted_insert(&mut arr, 3, 1, cmp), Some(3));
        assert_eq!(arr, vec![2]);
    }

    #[test]
    fn test_max_size_of_1_full_add_lesser() {
        let mut arr = vec![4];
        assert_eq!(sorted_insert(&mut arr, 3, 1, cmp), Some(4));
        assert_eq!(arr, vec![3]);
    }
}
