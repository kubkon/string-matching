use ::core::StringSearch;

static ALPHABET: isize = 256;

pub struct StringMatcher {
    prime: isize,
}

impl StringMatcher {
    pub fn new(prime: isize) -> StringMatcher {
        StringMatcher { prime: prime }
    }
}

impl StringSearch for StringMatcher {
    fn _search(&self, text: &str, pattern: &str) -> Vec<usize> {
        let mut indices = Vec::new();
        let m = text.len();
        let n = pattern.len();
        let h = (0 .. n - 1).fold(1, |acc, _| (acc * ALPHABET) % self.prime);
        // pre-processing
        let text_b: Vec<isize> = text.as_bytes().iter().map(|&x| x as isize).collect();
        let pattern_b: Vec<isize> = pattern.as_bytes().iter().map(|&x| x as isize).collect();
        let (p, mut t) = pattern_b.iter()
                                  .zip(text_b.iter())
                                  .fold((0, 0), |(p, t), (&x, &y)| (
                                      (p * ALPHABET + x) % self.prime,
                                      (t * ALPHABET + y) % self.prime,
                                  ));
        // matching
        for i in 0 .. m - n + 1 {
            if p == t {
                if &text[i .. n + i] == pattern {
                    indices.push(i);
                }
            }
            if i < m - n {
                t = (ALPHABET * (t - text_b[i] * h) + text_b[i + n]) % self.prime;
                t = if t < 0 { t + self.prime } else { t };
            }
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
        let matcher = StringMatcher::new(101);
        let expected: Vec<usize> = Vec::new();
        assert_eq!(expected, matcher.search("abc", "aaaa"));
        assert_eq!(expected, matcher.search("abc", ""));
        assert_eq!(expected, matcher.search("", "aaaa"));
        assert_eq!(expected, matcher.search("", ""));
    }

    #[test]
    fn valid_pattern() {
        let matcher = StringMatcher::new(101);
        assert_eq!(vec![3],   matcher.search("abcabaabcabac", "abaa"));
        assert_eq!(vec![0,6], matcher.search("abcabaabcabac", "abc"));
        assert_eq!(vec![3,9], matcher.search("abcabaabcabac", "aba"));
        assert_eq!(vec![0,3,10], matcher.search("abcabaacbaabc", "ab"));
        assert_eq!(vec![0,10], matcher.search("abcabaacbaabc", "abc"));
    }
}
