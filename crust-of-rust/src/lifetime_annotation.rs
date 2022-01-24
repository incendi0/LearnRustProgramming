pub struct StrSplit<'a, Delimiter> {
    remainder: Option<&'a str>,
    delimiter: Delimiter,
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'a, Delimiter> StrSplit<'a, Delimiter> {
    pub fn new(haystack: &'a str, delimiter: Delimiter) -> Self {
        StrSplit { 
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a, D: Delimiter> Iterator for StrSplit<'a, D> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delimt_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delimt_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices().find(|(_, c)| c == self).map(|(start, _)| (start, start + self.len_utf8()))
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c).next().expect("need at least one match result")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_works() {
        let mut ss = StrSplit::new("张 王 李 赵 刘 ", '李');
        assert_eq!(ss.next(), Some("张 王 "));
        assert_eq!(ss.next(), Some(" 赵 刘 "));
        assert_eq!(ss.next(), None);
    }

    #[test]
    fn iterator_works_2() {
        let mut ss = StrSplit::new("张 王 李 赵 刘 ", ' ');
        assert_eq!(ss.next(), Some("张"));
        assert_eq!(ss.next(), Some("王"));
        assert_eq!(ss.next(), Some("李"));
        assert_eq!(ss.next(), Some("赵"));
        assert_eq!(ss.next(), Some("刘"));
        assert_eq!(ss.next(), Some(""));
        assert_eq!(ss.next(), None);
    }

    #[test]
    fn until_char_works() {
        assert_eq!(until_char("hello world", 'o'), "hell");
        assert_eq!(until_char("y̆es", 'y'), "");
        // assert_eq!(until_char("y̆es", "y̆"), "");
    }
}
