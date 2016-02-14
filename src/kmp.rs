use std::cmp::max;
use ::core::StringSearch;

pub struct StringMatcher;

impl StringMatcher {
    pub fn new() -> StringMatcher {
        StringMatcher
    }
    
    fn compute_failure(&self, pattern: &str) -> Vec<usize> {
        let n = pattern.len();
        let mut shifts = Vec::with_capacity(n);
        shifts.push(0);
        let mut k = 0;
        let p: Vec<char> = pattern.chars().collect();
        for q in 1 .. n {
            loop {
                if k <= 0 || p[k] == p[q] {
                    break;
                }
                k = shifts[k];
            }
            
            if p[k] == p[q] {
                k = k + 1;
            }

            shifts.push(k);
        }
        shifts
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

    #[test]
    fn compute_failure() {
        let matcher = StringMatcher::new();
        assert_eq!(vec![0,0,1,2],       matcher.compute_failure("abab"));
        assert_eq!(vec![0,0,1,2,3,0,1], matcher.compute_failure("ababaca"));
    }
}
