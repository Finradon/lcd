use crate::data_loader::Spell;
use std::collections::HashMap;
use strsim::levenshtein;

/// Fuzzy searches for the best match by name using a given tolerance.
/// Supports full-string and multi-word queries.
pub fn fuzzy_search_best_match<'a>(
    spells: &'a HashMap<String, Spell>,
    query: &str,
    tolerance: usize
) -> Option<&'a String> {
    let query = query.to_lowercase();
    let mut best_match: Option<&String> = None;
    let mut best_distance = usize::MAX;

    for (name, _) in spells.iter() {
        let name_lower = name.to_lowercase();

        // Check if the full query is a substring of the spell name (case-insensitive)
        if name_lower.contains(&query) {
            return Some(name); // Return immediately if the query is found as a substring
        }

        // Split the spell name into words and find the minimum distance for any word
        for word in name_lower.split_whitespace() {
            let distance = levenshtein(&word, &query);
            if distance <= tolerance && distance < best_distance {
                best_match = Some(name);
                best_distance = distance;
            }
        }
    }

    best_match
}
