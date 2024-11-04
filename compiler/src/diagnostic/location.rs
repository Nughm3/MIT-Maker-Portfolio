use std::fmt;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    /// Zero-indexed line
    line: usize,
    /// Zero-indexed column
    col: usize,
}

impl Location {
    pub fn new(line: usize, col: usize) -> Self {
        Location { line, col }
    }

    pub fn from_index(contents: &str, line_breaks: &[usize], offset: usize) -> Self {
        assert!(offset < contents.len(), "index out of bounds");
        let line = line_breaks.partition_point(|&i| i < offset);
        if line == 0 {
            return Location::new(0, offset);
        }
        Location::new(line, offset - line_breaks[line - 1] - 1)
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.col + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(contents: &str, offset: usize) -> Location {
        let line_breaks: Vec<_> = contents
            .char_indices()
            .filter_map(|(i, c)| (c == '\n').then_some(i))
            .collect();
        Location::from_index(contents, &line_breaks, offset)
    }

    #[test]
    fn single_line() {
        let contents = "Hello, world!";
        assert_eq!(check(contents, 0), Location::new(0, 0));
        assert_eq!(check(contents, 7), Location::new(0, 7));
        assert_eq!(check(contents, 12), Location::new(0, 12));
    }

    #[test]
    fn multiple_lines() {
        let contents = "Hello,\nWorld!\n";
        assert_eq!(check(contents, 0), Location::new(0, 0));
        assert_eq!(check(contents, 5), Location::new(0, 5));
        assert_eq!(check(contents, 6), Location::new(0, 6));
        assert_eq!(check(contents, 7), Location::new(1, 0));
        assert_eq!(check(contents, 12), Location::new(1, 5));
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        check("Line 1\nLine 2", 13);
    }

    #[test]
    fn edge_cases() {
        let contents = "Line 1\nLine 2\n";
        assert_eq!(check(contents, 7), Location::new(1, 0));
        assert_eq!(check(contents, 12), Location::new(1, 5));
    }
}
