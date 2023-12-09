pub fn sorted_insert<T: Ord>(items: &mut Vec<T>, add: T, max_size: usize) -> Option<T> {
    assert!(max_size > 0, "MAX_SIZE_ZERO");
    assert!(items.len() <= max_size, "ITEMS_SIZE");

    // Find the position to insert the new item
    let pos = match items.binary_search(&add) {
        Ok(pos) | Err(pos) => pos,
    };

    // If the array is full, remove the last item
    let removed_item = if items.len() == max_size {
        items.pop()
    } else {
        None
    };

    // Insert the new item at the correct position
    items.insert(pos, add);

    removed_item
}
