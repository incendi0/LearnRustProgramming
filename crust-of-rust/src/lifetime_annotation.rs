pub struct StrSplit<'a, 'b> {
    remainder: Option<&'a str>,
    delimiter: &'b str,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        StrSplit { 
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
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

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &c.to_string()).next().unwrap()
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

    #[test]
    fn until_char_works() {
        assert_eq!(until_char("hello world", 'o'), "hell");
    }
}
