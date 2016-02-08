use ::core::StringSearch;

pub struct StringMatcher;

impl StringMatcher {
    pub fn new() -> StringMatcher {
        StringMatcher
    }
}

impl StringSearch for StringMatcher {
    fn _search(&self, text: &str, pattern: &str) -> Vec<usize> {
        let mut indices = Vec::new();
        let m = text.len();
        let n = pattern.len();
        for i in 0 .. m - n + 1 {
            if &text[i .. n + i] == pattern {
                indices.push(i);
            }
        }
        indices
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
        assert_eq!(vec![3],      matcher.search("abcabaabcabac", "abaa"));
        assert_eq!(vec![0,6],    matcher.search("abcabaabcabac", "abc"));
        assert_eq!(vec![3,9],    matcher.search("abcabaabcabac", "aba"));
        assert_eq!(vec![0,3,10], matcher.search("abcabaacbaabc", "ab"));
        assert_eq!(vec![0,10],   matcher.search("abcabaacbaabc", "abc"));
    }
}
