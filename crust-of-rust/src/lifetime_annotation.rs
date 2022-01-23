pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        StrSplit { 
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(delim_idx) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..delim_idx];
                *remainder = &remainder[delim_idx + self.delimiter.len()..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_works() {
        let mut ss = StrSplit::new("a b c d e ", " ");
        assert_eq!(ss.next(), Some("a"));
        assert_eq!(ss.next(), Some("b"));
        assert_eq!(ss.next(), Some("c"));
        assert_eq!(ss.next(), Some("d"));
        assert_eq!(ss.next(), Some("e"));
        assert_eq!(ss.next(), Some(""));
        assert_eq!(ss.next(), None);
    }
}