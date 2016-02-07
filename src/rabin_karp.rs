use ::core::StringSearch;

pub struct StringMatcher {
    alphabet_size: u32,
    modulus_prime: u32,
}

impl StringMatcher {
    pub fn new(alphabet_size: u32, modulus_prime: u32) -> StringMatcher {
        StringMatcher {
            alphabet_size: alphabet_size,
            modulus_prime: modulus_prime,
        }
    }
}

impl StringSearch for StringMatcher {
    fn _search(&self, text: &str, pattern: &str) -> Vec<usize> {
        let mut indices = Vec::new();
        let m = text.len();
        let n = pattern.len();
        let h = self.alphabet_size.pow(n as u32 - 1) % self.modulus_prime;
        // pre-processing
        let text_b: Vec<u32> = text.as_bytes().iter().map(|&x| x as u32).collect();
        let pattern_b: Vec<u32> = pattern.as_bytes().iter().map(|&x| x as u32).collect();
        let (p, mut t) = pattern_b.iter()
                                  .zip(text_b.iter())
                                  .fold((0,0), |(p, t), (&x, &y)| (
                                      (p * self.alphabet_size + x) % self.modulus_prime,
                                      (t * self.alphabet_size + y) % self.modulus_prime,
                                  ));
        // matching
        for i in 0 .. m - n {
            if p == t {
                if &text[i .. n + i] == pattern {
                    indices.push(i);
                }
            }
            t = (self.alphabet_size * (t - h * text_b[i]) + text_b[i + n]) % self.modulus_prime
        }
        indices
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
        assert_eq!(vec![0,3,10], matcher.search("abcabaacbaabc", "ab"));
        assert_eq!(vec![0,10], matcher.search("abcabaacbaabc", "abc"));
    }
}
