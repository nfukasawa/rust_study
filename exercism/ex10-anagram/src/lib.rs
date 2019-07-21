use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let w = Word::new(word);
    possible_anagrams
        .iter()
        .filter(|s| w.is_anagram(s))
        .cloned()
        .collect()
}

struct Word {
    sorted: String,
    low: String,
}

impl Word {
    pub fn new(word: &str) -> Word {
        let l = word.to_lowercase();
        Word {
            sorted: sort(&l),
            low: l,
        }
    }
    pub fn is_anagram(&self, s: &str) -> bool {
        let l = s.to_lowercase();
        self.low != l && self.sorted == sort(&l)
    }
}


fn sort(s: &String) -> String {
    let mut sorted = s.chars().collect::<Vec<char>>();
    sorted.sort();
    sorted.iter().collect()
}
