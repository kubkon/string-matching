use ::core::StringSearch;

pub struct StringMatcher {
    alphabet_size: usize,
    modulus_prime: usize,
}

impl StringMatcher {
    pub fn new(alphabet_size: usize, modulus_prime: usize) -> StringMatcher {
        StringMatcher {
            alphabet_size: alphabet_size,
            modulus_prime: modulus_prime,
        }
    }
}

impl StringSearch for StringMatcher {
    fn _search(&self, text: &str, pattern: &str) -> Vec<usize> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use ::core::StringSearch;
    use ::rabin_karp::StringMatcher;

    #[test]
    fn invalid_pattern() {
        let matcher = StringMatcher::new(26, 13);
        let expected: Vec<usize> = Vec::new();
        assert_eq!(expected, matcher.search("abc", "aaaa"));
        assert_eq!(expected, matcher.search("abc", ""));
        assert_eq!(expected, matcher.search("", "aaaa"));
        assert_eq!(expected, matcher.search("", ""));
    }

    #[test]
    fn valid_pattern() {
        let matcher = StringMatcher::new(26, 13);
        assert_eq!(vec![3],   matcher.search("abcabaabcabac", "abaa"));
        assert_eq!(vec![0,6], matcher.search("abcabaabcabac", "abc"));
        assert_eq!(vec![3,9], matcher.search("abcabaabcabac", "aba"));
    }
}
