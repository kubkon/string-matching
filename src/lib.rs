pub struct NaiveStringMatcher {
    text: String,
}

impl NaiveStringMatcher {
    pub fn new(text: &str) -> NaiveStringMatcher {
        NaiveStringMatcher { text: String::from(text) }
    }

    pub fn search(&self, pattern: &str) -> Vec<usize> {
        match (self.text.len(), pattern.len()) {
            (0, _) | (_, 0) => Vec::new(),
            (m, n) if m >= n => {
                let mut indices = Vec::new();
                for i in 0 .. m - n {
                    if &self.text[i .. n + i] == pattern {
                        indices.push(i);
                    }
                }
                indices
            }
            _ => Vec::new(),
        }
    }
}

#[test]
fn invalid_pattern() {
    let mut matcher = NaiveStringMatcher::new("abc");
    let expected: Vec<usize> = Vec::new();
    assert_eq!(expected, matcher.search("aaaa"));
    assert_eq!(expected, matcher.search(""));
    
    matcher = NaiveStringMatcher::new("");
    assert_eq!(expected, matcher.search("aaaa"));
    assert_eq!(expected, matcher.search(""));
}

#[test]
fn valid_pattern() {
    let matcher = NaiveStringMatcher::new("abcabaabcabac");
    assert_eq!(vec![3], matcher.search("abaa"));
    assert_eq!(vec![0,6], matcher.search("abc"));
    assert_eq!(vec![3,9], matcher.search("aba"));
}
