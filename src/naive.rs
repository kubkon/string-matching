use ::core::StringMatcher;

pub struct NaiveStringMatcher;

impl StringMatcher for NaiveStringMatcher {
    fn _search(&self, text: &str, pattern: &str) -> Vec<usize> {
        let mut indices = Vec::new();
        let m = text.len();
        let n = pattern.len();
        for i in 0 .. m - n {
            if &text[i .. n + i] == pattern {
                indices.push(i);
            }
        }
        indices
    }
}

#[cfg(test)]
mod tests {
    use ::NaiveStringMatcher;
    use ::core::StringMatcher;

    #[test]
    fn invalid_pattern() {
        let expected: Vec<usize> = Vec::new();
        assert_eq!(expected, NaiveStringMatcher.search("abc", "aaaa"));
        assert_eq!(expected, NaiveStringMatcher.search("abc", ""));
        
        assert_eq!(expected, NaiveStringMatcher.search("", "aaaa"));
        assert_eq!(expected, NaiveStringMatcher.search("", ""));
    }

    #[test]
    fn valid_pattern() {
        assert_eq!(vec![3], NaiveStringMatcher.search("abcabaabcabac", "abaa"));
        assert_eq!(vec![0,6], NaiveStringMatcher.search("abcabaabcabac", "abc"));
        assert_eq!(vec![3,9], NaiveStringMatcher.search("abcabaabcabac", "aba"));
    }
}
