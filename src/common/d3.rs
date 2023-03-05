use std::collections::HashSet;

pub fn parse_items_to_priorities(items: &HashSet<char>) -> HashSet<u8> {
    items
        .iter()
        .map(|item| {
            if *item as u8 >= b'a' {
                (*item as u8 - b'a') + 1
            } else {
                (*item as u8 - b'A') + 27
            }
        })
        .collect()
}
