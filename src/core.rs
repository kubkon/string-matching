pub trait StringSearch {
    fn _search(&self, text: &str, pattern: &str) -> Vec<usize>;

    fn search(&self, text: &str, pattern: &str) -> Vec<usize> {
        match (text.len(), pattern.len()) {
            (0, _) | (_, 0)  => Vec::new(),
            (m, n) if m >= n => self._search(text, pattern),
            _                => Vec::new(),
        }
    }
}

