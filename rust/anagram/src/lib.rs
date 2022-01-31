use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let w_len = word.len();
    let w_lower = word.to_lowercase();
    let w_chars = transform(word);

    possible_anagrams
        .iter()
        .map(|o| *o)
        .filter(|o| w_len == o.len() && w_lower != o.to_lowercase() && w_chars == transform(o))
        .collect()
}

fn transform(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable();

    chars
}
