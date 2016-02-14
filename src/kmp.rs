use ::core::StringSearch;

pub struct StringMatcher;

impl StringMatcher {
    pub fn new() -> StringMatcher {
        StringMatcher
    }
    
    fn compute_shifts(&self, pattern: &str) -> Vec<usize> {
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
        let mut indices = Vec::new();
        let shifts = self.compute_shifts(pattern);
        let t: Vec<char> = text.chars().collect();
        let p: Vec<char> = pattern.chars().collect();
        let m = t.len();
        let n = p.len();
        let mut q = 0;
        for i in 0 .. m {
            loop {
                if q <= 0 || p[q] == t[i] {
                    break;
                }
                q = shifts[q];
            }

            if p[q] == t[i] {
                q = q + 1;
            }

            if q == n {
                let index = i as isize - n as isize + 1;
                indices.push(index as usize);
                q = shifts[q - 1];
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

    #[test]
    fn compute_shifts() {
        let matcher = StringMatcher::new();
        assert_eq!(vec![0,0,1,2],       matcher.compute_shifts("abab"));
        assert_eq!(vec![0,0,1,2,3,0,1], matcher.compute_shifts("ababaca"));
    }
}
