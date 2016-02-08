use ::core::StringSearch;
use std::collections::HashMap;
use std::cmp::min;

static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub struct StringMatcher;

impl StringMatcher {
    pub fn new() -> StringMatcher {
        StringMatcher
    }

    fn construct_dfa(&self, pattern: &str) -> HashMap<(usize, char), usize> {
        let mut transitions = HashMap::new();
        let n = pattern.len();
        for state in 0 .. n + 1 {
            let prefix = &pattern[0 .. state];
            for c in ALPHABET.chars() {
                let mut k = min(n + 1, state + 2);
                loop {
                    k = k - 1;
                    let suffix = &pattern[0 .. k];
                    let phrase = prefix.to_string() + &c.to_string();
                    let m = phrase.len();
                    if suffix == &phrase[m - suffix.len() .. m] {
                        break;
                    }
                }
                transitions.insert((state, c), k);
            }
        }
        transitions
    }
}

impl StringSearch for StringMatcher {
    fn _search(&self, text: &str, pattern: &str) -> Vec<usize> {
        let transitions = self.construct_dfa(pattern);
        println!("{:?}", transitions);
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use ::core::StringSearch;
    use super::StringMatcher;

    #[test]
    fn invalid_pattern() {
        let matcher = StringMatcher::new();
        let expected: Vec<usize> = Vec::new();
        assert_eq!(expected, matcher.search("abc", "aaaa"));
        assert_eq!(expected, matcher.search("abc", ""));
        assert_eq!(expected, matcher.search("", "aaaa"));
        assert_eq!(expected, matcher.search("", ""));
    }

    #[test]
    fn valid_pattern() {
        let matcher = StringMatcher::new();
        assert_eq!(vec![3],   matcher.search("abcabaabcabac", "abaa"));
        assert_eq!(vec![0,6], matcher.search("abcabaabcabac", "abc"));
        assert_eq!(vec![3,9], matcher.search("abcabaabcabac", "aba"));
        assert_eq!(vec![0,3,10], matcher.search("abcabaacbaabc", "ab"));
        assert_eq!(vec![0,10], matcher.search("abcabaacbaabc", "abc"));
    }
}
