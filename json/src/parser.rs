use crate::{error::JsonError, value::JsonValue, Result};
use std::{collections::HashMap, str};

struct Parser<'a> {
    content: &'a [u8],
    index: usize,
    length: usize,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        let content = content.as_bytes();
        Parser {
            content,
            index: 0,
            length: content.len(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.index < self.length
            && (self.content[self.index] == b' '
                || self.content[self.index] == b'\t'
                || self.content[self.index] == b'\n'
                || self.content[self.index] == b'\r')
        {
            self.index += 1;
        }
    }

    fn peek_once(&self) -> Option<u8> {
        if self.index < self.length {
            Some(self.content[self.index])
        } else {
            None
        }
    }

    fn next_once(&mut self) -> Option<u8> {
        if self.index < self.length {
            self.index += 1;
            Some(self.content[self.index - 1])
        } else {
            None
        }
    }

    fn consume_once(&mut self) {
        self.index += 1;
    }

    fn parse_value(&mut self) -> Result<JsonValue> {
        match self.peek_once() {
            Some(ch) if ch == b't' || ch == b'f' || ch == b'n' => self.parse_literal(ch).map(|s| {
                if s == "null" {
                    JsonValue::Null
                } else if s == "true" {
                    JsonValue::Bool(true)
                } else {
                    JsonValue::Bool(false)
                }
            }),
            Some(ch) if ch == b'"' => self.parse_string().map(|s| JsonValue::String(s)),
            Some(ch) if ch == b'[' => self.parse_array().map(|arr| JsonValue::Array(arr)),
            Some(ch) if ch == b'{' => self.parse_object().map(|m| JsonValue::Object(m)),
            Some(_) => self.parse_number(),
            None => Err(JsonError::UnexpectedValue),
        }
    }

    fn parse_object(&mut self) -> Result<HashMap<String, JsonValue>> {
        self.consume_once();
        self.skip_whitespace();
        let mut attrs = HashMap::new();
        if let Some(b'}') = self.peek_once() {
            self.consume_once();
            return Ok(attrs);
        }
        loop {
            if self.index >= self.length || self.content[self.index] != b'\"' {
                return Err(JsonError::MissEntryKey);
            }
            let k = self.parse_string()?;
            self.skip_whitespace();
            if self.index >= self.length || self.content[self.index] != b':' {
                return Err(JsonError::MissEntryColon);
            }
            self.consume_once();
            self.skip_whitespace();
            let v = self.parse_value()?;
            self.skip_whitespace();
            attrs.insert(k, v);
            match self.next_once() {
                Some(b'}') => {
                    return Ok(attrs);
                }
                Some(b',') => {
                    self.skip_whitespace();
                }
                _ => return Err(JsonError::MissCommaOrRightCurlyBracket),
            }
        }
    }

    fn parse_array(&mut self) -> Result<Vec<JsonValue>> {
        // consume [
        self.consume_once();
        self.skip_whitespace();
        let mut arr = vec![];
        if let Some(b']') = self.peek_once() {
            self.consume_once();
            return Ok(arr);
        }
        loop {
            let v = self.parse_value()?;
            self.skip_whitespace();
            arr.push(v);
            match self.next_once() {
                Some(b']') => {
                    return Ok(arr);
                }
                Some(b',') => {
                    self.skip_whitespace();
                }
                _ => return Err(JsonError::MissCommaOrRightSquareBracket),
            }
        }
    }

    fn parse_string(&mut self) -> Result<String> {
        // consume "
        self.consume_once();
        let mut s = String::new();
        loop {
            match self.next_once() {
                Some(b'\"') => {
                    return Ok(s);
                }
                Some(b'\\') => match self.next_once() {
                    Some(b'\"') => s.push('\"'),
                    Some(b'\\') => s.push('\\'),
                    Some(b'/') => s.push('/'),
                    // see http://man-ascii.com/
                    Some(b'b') => s.push('\x08'),
                    Some(b'f') => s.push('\x0b'),
                    Some(b'n') => s.push('\n'),
                    Some(b'r') => s.push('\r'),
                    Some(b't') => s.push('\t'),
                    Some(b'u') => {
                        let mut u = self.parse_unicode()?;
                        // 处理高低代理对
                        if (0xD800..=0xDBFF).contains(&u) {
                            if Some(b'\\') != self.next_once() || Some(b'u') != self.next_once() {
                                return Err(JsonError::InvalidUnicodeSurrogate);
                            }
                            let u2 = self
                                .parse_unicode()
                                .map_err(|_| JsonError::InvalidUnicodeSurrogate)?;
                            if u2 < 0xDC00 || u2 > 0xDFFF {
                                return Err(JsonError::InvalidUnicodeSurrogate);
                            }
                            u = (((u - 0xD800) << 10) | (u2 - 0xDC00)) + 0x10000;
                        }
                        // 很幸运，我们有char::from_u32
                        if let Some(ch) = char::from_u32(u) {
                            s.push(ch)
                        } else {
                            return Err(JsonError::InvalidUnicodeSurrogate);
                        }
                    }
                    _ => return Err(JsonError::InvalidStringEscape),
                },
                Some(ch) if ch >= 0x20 => s.push(ch as char),
                Some(_) => return Err(JsonError::InvalidStringChar),
                None => return Err(JsonError::MissEndQuotationMark),
            }
        }
    }

    // \uxxxx
    fn parse_unicode(&mut self) -> Result<u32> {
        if self.index + 4 >= self.length {
            return Err(JsonError::InvalidUnicodeHex);
        }
        let mut u: u32 = 0;
        for _ in 0..4 {
            let b = self.content[self.index];
            u <<= 4;
            if Self::is_digit_0_to_9(b) {
                u |= (b - b'0') as u32;
            } else if Self::is_digit_in_range(b, b'a', b'f') {
                u |= (b - b'a' + 10) as u32;
            } else if Self::is_digit_in_range(b, b'A', b'F') {
                u |= (b - b'A' + 10) as u32;
            } else {
                return Err(JsonError::InvalidUnicodeHex);
            }
            self.index += 1;
        }
        Ok(u)
    }

    fn parse_literal(&mut self, st: u8) -> Result<&str> {
        let literal = if st == b't' {
            ("true", 4)
        } else if st == b'f' {
            ("false", 5)
        } else {
            ("null", 4)
        };
        if self.index + literal.1 <= self.length
            && &self.content[self.index..self.index + literal.1] == literal.0.as_bytes()
        {
            self.index += literal.1;
            Ok(literal.0)
        } else {
            Err(JsonError::UnexpectedValue)
        }
    }

    fn is_digit_in_range(digit: u8, start: u8, end: u8) -> bool {
        digit >= start && digit <= end
    }

    fn is_digit_1_to_9(digit: u8) -> bool {
        (b'1'..=b'9').contains(&digit)
    }

    fn is_digit_0_to_9(digit: u8) -> bool {
        (b'0'..=b'9').contains(&digit)
    }

    // 此处check浮点数是否合法，数值大小交给f64::parse
    // number = [ minus ] int [ frac ] [ exp ]
    // decimal-point = %x2E       ; .
    // digit1-9 = %x31-39         ; 1-9
    // e = %x65 / %x45            ; e E
    // exp = e [ minus / plus ] 1*DIGIT
    // frac = decimal-point 1*DIGIT
    // int = zero / ( digit1-9 *DIGIT )
    // minus = %x2D               ; -
    // plus = %x2B                ; +
    // zero = %x30                ; 0
    fn parse_number(&mut self) -> Result<JsonValue> {
        let start = self.index;
        // optional minus
        let mut has_minus = false;
        if self.content[self.index] == b'-' {
            has_minus = true;
            self.index += 1;
        }
        if self.index >= self.length {
            return Err(JsonError::InvalidNumber);
        }
        // required int
        if self.content[self.index] == b'0' {
            self.index += 1;
        } else if !Self::is_digit_1_to_9(self.content[self.index]) {
            return if has_minus {
                Err(JsonError::InvalidNumber)
            } else {
                Err(JsonError::UnexpectedValue)
            };
        } else {
            loop {
                if self.index >= self.length || !Self::is_digit_0_to_9(self.content[self.index]) {
                    break;
                }
                self.index += 1;
            }
        }

        // optional frac
        match self.peek_once() {
            Some(ch) if ch == b'.' => {
                self.consume_once();
                if self.index >= self.length {
                    return Err(JsonError::InvalidNumber);
                } else {
                    // one or more digit
                    if !Self::is_digit_0_to_9(self.content[self.index]) {
                        return Err(JsonError::InvalidNumber);
                    } else {
                        loop {
                            if self.index >= self.length
                                || !Self::is_digit_0_to_9(self.content[self.index])
                            {
                                break;
                            }
                            self.index += 1;
                        }
                    }
                }
            }
            _ => {}
        }
        // optional exp
        match self.peek_once() {
            Some(ch) if ch == b'e' || ch == b'E' => {
                self.consume_once();
                if self.index >= self.length {
                    return Err(JsonError::InvalidNumber);
                }
                // optional minus or plus
                if self.content[self.index] == b'+' || self.content[self.index] == b'-' {
                    self.consume_once();
                }
                if self.index >= self.length {
                    return Err(JsonError::InvalidNumber);
                }
                // one or more digit
                if !Self::is_digit_0_to_9(self.content[self.index]) {
                    return Err(JsonError::InvalidNumber);
                } else {
                    loop {
                        if self.index >= self.length
                            || !Self::is_digit_0_to_9(self.content[self.index])
                        {
                            break;
                        }
                        self.index += 1;
                    }
                }
            }
            _ => {}
        }
        if let Ok(num_str) = str::from_utf8(&self.content[start..self.index]) {
            if let Ok(num) = num_str.parse::<f64>() {
                // num could be inf, nan
                if !num.is_infinite() {
                    return Ok(JsonValue::Number(num));
                }
            }
        }
        Err(JsonError::InvalidNumber)
    }

    pub fn parse(&mut self) -> Result<JsonValue> {
        self.skip_whitespace();
        let v = self.parse_value()?;
        self.skip_whitespace();
        if self.index < self.length {
            Err(JsonError::RootNotSingular)
        } else {
            Ok(v)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_literal_works() {
        assert_eq!(Parser::new("null").parse(), Ok(JsonValue::Null));
        assert_eq!(Parser::new("   \t\nnull").parse(), Ok(JsonValue::Null));
        assert_eq!(Parser::new("null  \r").parse(), Ok(JsonValue::Null));
        assert_eq!(
            Parser::new(" \ttrue\r\n").parse(),
            Ok(JsonValue::Bool(true))
        );
        assert_eq!(Parser::new("true").parse(), Ok(JsonValue::Bool(true)));
        assert_eq!(Parser::new(" false ").parse(), Ok(JsonValue::Bool(false)));
        assert_eq!(Parser::new("false").parse(), Ok(JsonValue::Bool(false)));
    }

    #[test]
    fn parse_invalid_literal_works() {
        assert_eq!(Parser::new("nul").parse(), Err(JsonError::UnexpectedValue));
        assert_eq!(
            Parser::new("nul l").parse(),
            Err(JsonError::UnexpectedValue)
        );
        assert_eq!(
            Parser::new("float").parse(),
            Err(JsonError::UnexpectedValue)
        );
        assert_eq!(Parser::new("time").parse(), Err(JsonError::UnexpectedValue));
    }

    #[test]
    fn parser_number_works() {
        assert_eq!(Parser::new("0").parse(), Ok(JsonValue::Number(0f64)));
        assert_eq!(Parser::new("-0").parse(), Ok(JsonValue::Number(0f64)));
        assert_eq!(Parser::new("-0.0").parse(), Ok(JsonValue::Number(0f64)));
        assert_eq!(Parser::new("1").parse(), Ok(JsonValue::Number(1f64)));
        assert_eq!(Parser::new("-1").parse(), Ok(JsonValue::Number(-1f64)));
        assert_eq!(Parser::new("1.5").parse(), Ok(JsonValue::Number(1.5f64)));
        assert_eq!(Parser::new("-1.5").parse(), Ok(JsonValue::Number(-1.5f64)));
        assert_eq!(
            Parser::new("3.1416").parse(),
            Ok(JsonValue::Number(3.1416f64))
        );
        assert_eq!(Parser::new("1E10").parse(), Ok(JsonValue::Number(1E10f64)));
        assert_eq!(Parser::new("1e10").parse(), Ok(JsonValue::Number(1e10f64)));
        assert_eq!(
            Parser::new("1E+10").parse(),
            Ok(JsonValue::Number(1E+10f64))
        );
        assert_eq!(
            Parser::new("1E-10").parse(),
            Ok(JsonValue::Number(1E-10f64))
        );
        assert_eq!(
            Parser::new("-1E10").parse(),
            Ok(JsonValue::Number(-1E10f64))
        );
        assert_eq!(
            Parser::new("-1e10").parse(),
            Ok(JsonValue::Number(-1e10f64))
        );
        assert_eq!(
            Parser::new("-1E+10").parse(),
            Ok(JsonValue::Number(-1E+10f64))
        );
        assert_eq!(
            Parser::new("-1E-10").parse(),
            Ok(JsonValue::Number(-1E-10f64))
        );
        assert_eq!(
            Parser::new("1.234E+10").parse(),
            Ok(JsonValue::Number(1.234E+10f64))
        );
        assert_eq!(
            Parser::new("1.234E-10").parse(),
            Ok(JsonValue::Number(1.234E-10f64))
        );
        assert_eq!(Parser::new("1e-10000").parse(), Ok(JsonValue::Number(0f64)));
        assert_eq!(
            Parser::new("1.0000000000000002").parse(),
            Ok(JsonValue::Number(1.0000000000000002f64))
        );
        assert_eq!(
            Parser::new("4.9406564584124654e-324").parse(),
            Ok(JsonValue::Number(4.9406564584124654e-324f64))
        );
        assert_eq!(
            Parser::new("-4.9406564584124654e-324").parse(),
            Ok(JsonValue::Number(-4.9406564584124654e-324f64))
        );
        assert_eq!(
            Parser::new("2.2250738585072009e-308").parse(),
            Ok(JsonValue::Number(2.2250738585072009e-308f64))
        );
        assert_eq!(
            Parser::new("-2.2250738585072009e-308").parse(),
            Ok(JsonValue::Number(-2.2250738585072009e-308f64))
        );
        assert_eq!(
            Parser::new("2.2250738585072014e-308").parse(),
            Ok(JsonValue::Number(2.2250738585072014e-308f64))
        );
        assert_eq!(
            Parser::new("-2.2250738585072014e-308").parse(),
            Ok(JsonValue::Number(-2.2250738585072014e-308f64))
        );
        assert_eq!(
            Parser::new("1.7976931348623157e+308").parse(),
            Ok(JsonValue::Number(1.7976931348623157e+308f64))
        );
        assert_eq!(
            Parser::new("-1.7976931348623157e+308").parse(),
            Ok(JsonValue::Number(-1.7976931348623157e+308f64))
        );
    }

    #[test]
    fn parse_invalid_number_works() {
        // 格式错误
        assert_eq!(Parser::new("+").parse(), Err(JsonError::UnexpectedValue));
        assert_eq!(Parser::new("-").parse(), Err(JsonError::InvalidNumber));
        assert_eq!(Parser::new("+0").parse(), Err(JsonError::UnexpectedValue));
        assert_eq!(Parser::new("+1").parse(), Err(JsonError::UnexpectedValue));
        assert_eq!(Parser::new(".123").parse(), Err(JsonError::UnexpectedValue));
        assert_eq!(Parser::new("1.").parse(), Err(JsonError::InvalidNumber));
        assert_eq!(Parser::new("1.1e").parse(), Err(JsonError::InvalidNumber));
        assert_eq!(Parser::new("1.1e+").parse(), Err(JsonError::InvalidNumber));
        assert_eq!(Parser::new("1.1e-").parse(), Err(JsonError::InvalidNumber));
        assert_eq!(Parser::new("e12").parse(), Err(JsonError::UnexpectedValue));
        assert_eq!(Parser::new("1.e12").parse(), Err(JsonError::InvalidNumber));
        assert_eq!(Parser::new("inf").parse(), Err(JsonError::UnexpectedValue));
        assert_eq!(Parser::new("INF").parse(), Err(JsonError::UnexpectedValue));
        assert_eq!(Parser::new("NAN").parse(), Err(JsonError::UnexpectedValue));

        // 走到了null判断
        assert_eq!(Parser::new("nan").parse(), Err(JsonError::UnexpectedValue));

        // 数字太大
        assert_eq!(Parser::new("1e309").parse(), Err(JsonError::InvalidNumber));
        assert_eq!(Parser::new("-1e309").parse(), Err(JsonError::InvalidNumber));
    }

    #[test]
    fn parse_singular_err_works() {
        assert_eq!(
            Parser::new("null false").parse(),
            Err(JsonError::RootNotSingular)
        );
        assert_eq!(
            Parser::new("null family").parse(),
            Err(JsonError::RootNotSingular)
        );
        assert_eq!(
            Parser::new("0x123").parse(),
            Err(JsonError::RootNotSingular)
        );
        assert_eq!(Parser::new("0x0").parse(), Err(JsonError::RootNotSingular));
        assert_eq!(Parser::new("0123").parse(), Err(JsonError::RootNotSingular));
    }

    #[test]
    fn parse_miss_end_quotation_mark_works() {
        assert_eq!(
            Parser::new("\"").parse(),
            Err(JsonError::MissEndQuotationMark)
        );
        assert_eq!(
            Parser::new("\"abc").parse(),
            Err(JsonError::MissEndQuotationMark)
        );
    }

    #[test]
    fn parse_invalid_string_escape_works() {
        assert_eq!(
            Parser::new("\"\\v\"").parse(),
            Err(JsonError::InvalidStringEscape)
        );
        assert_eq!(
            Parser::new("\"\\'\"").parse(),
            Err(JsonError::InvalidStringEscape)
        );
        assert_eq!(
            Parser::new("\"\\0\"").parse(),
            Err(JsonError::InvalidStringEscape)
        );
        assert_eq!(
            Parser::new("\"\\x12\"").parse(),
            Err(JsonError::InvalidStringEscape)
        );
    }

    #[test]
    fn parse_invalid_string_char_works() {
        assert_eq!(
            Parser::new("\"\x01\"").parse(),
            Err(JsonError::InvalidStringChar)
        );
        assert_eq!(
            Parser::new("\"\x1F\"").parse(),
            Err(JsonError::InvalidStringChar)
        );
    }

    #[test]
    fn parse_invalid_unicode_hex_works() {
        assert_eq!(
            Parser::new("\"\\u\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u0\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u01\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u012\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u/000\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\uG000\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u0/00\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u0G00\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u00/0\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u00G0\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u000/\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u000G\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
        assert_eq!(
            Parser::new("\"\\u 123\"").parse(),
            Err(JsonError::InvalidUnicodeHex)
        );
    }

    #[test]
    fn parse_invalid_unicode_surrogate_works() {
        assert_eq!(
            Parser::new("\"\\uD800\"").parse(),
            Err(JsonError::InvalidUnicodeSurrogate)
        );
        assert_eq!(
            Parser::new("\"\\uDBFF\"").parse(),
            Err(JsonError::InvalidUnicodeSurrogate)
        );
        assert_eq!(
            Parser::new("\"\\uD800\\\\\"").parse(),
            Err(JsonError::InvalidUnicodeSurrogate)
        );
        assert_eq!(
            Parser::new("\"\\uD800\\uDBFF\"").parse(),
            Err(JsonError::InvalidUnicodeSurrogate)
        );
        assert_eq!(
            Parser::new("\"\\uD800\\uE000\"").parse(),
            Err(JsonError::InvalidUnicodeSurrogate)
        );
    }

    #[test]
    fn parse_unicode_works() {
        assert_eq!(
            Parser::new("\"\\u4e2d\\u56fd\"").parse(),
            Ok(JsonValue::String("中国".into()))
        );
        assert_eq!(
            Parser::new("\"\\u7b14\\u82af\\u2606\"").parse(),
            Ok(JsonValue::String("笔芯☆".into()))
        );
    }

    #[test]
    fn parse_string_works() {
        assert_eq!(
            Parser::new("\"\"").parse(),
            Ok(JsonValue::String("".into()))
        );
        assert_eq!(
            Parser::new("\"Hello\"").parse(),
            Ok(JsonValue::String("Hello".into()))
        );
        assert_eq!(
            Parser::new("\"Hello\\nWorld\"").parse(),
            Ok(JsonValue::String("Hello\nWorld".into()))
        );
        assert_eq!(
            Parser::new("\"\\\" \\\\ \\/ \\b \\f \\n \\r \\t\"").parse(),
            Ok(JsonValue::String("\" \\ / \x08 \x0b \n \r \t".into()))
        );
        assert_eq!(
            Parser::new("\"Hello\\u0000World\"").parse(),
            Ok(JsonValue::String("Hello\0World".into()))
        );
        assert_eq!(
            Parser::new("\"\\u0024\"").parse(),
            Ok(JsonValue::String("\x24".into()))
        );
        assert_eq!(
            Parser::new("\"\\u00A2\"").parse(),
            Ok(JsonValue::String("¢".into()))
        );
        assert_eq!(
            Parser::new("\"\\u20AC\"").parse(),
            Ok(JsonValue::String("€".into()))
        );
        assert_eq!(
            Parser::new("\"\\uD834\\uDD1E\"").parse(),
            Ok(JsonValue::String("𝄞".into()))
        );
        assert_eq!(
            Parser::new("\"\\ud834\\udd1e\"").parse(),
            Ok(JsonValue::String("𝄞".into()))
        );
    }

    #[test]
    fn parse_unexpected_value_works() {
        assert_eq!(Parser::new(" ").parse(), Err(JsonError::UnexpectedValue));
        assert_eq!(Parser::new("").parse(), Err(JsonError::UnexpectedValue));
        assert_eq!(
            Parser::new("[1, ]").parse(),
            Err(JsonError::UnexpectedValue)
        );
        assert_eq!(
            Parser::new("[\"a\", nul]").parse(),
            Err(JsonError::UnexpectedValue)
        );
    }

    #[test]
    fn parse_array_works_1() {
        assert_eq!(
            Parser::new("[ null , false , true , 3.1415926 , \"abc\" ]").parse(),
            Ok(JsonValue::Array(vec![
                JsonValue::Null,
                JsonValue::Bool(false),
                JsonValue::Bool(true),
                JsonValue::Number(3.1415926),
                JsonValue::String("abc".into())
            ]))
        )
    }

    #[test]
    fn parse_array_works_2() {
        assert_eq!(
            Parser::new("[ [ ] , [ 0 ] , [ 0 , 1 ] , [ 0 , 1 , 2 ] ]").parse(),
            Ok(JsonValue::Array(vec![
                JsonValue::Array(vec![]),
                JsonValue::Array(vec![JsonValue::Number(0f64)]),
                JsonValue::Array(vec![JsonValue::Number(0f64), JsonValue::Number(1f64)]),
                JsonValue::Array(vec![
                    JsonValue::Number(0f64),
                    JsonValue::Number(1f64),
                    JsonValue::Number(2f64)
                ]),
            ]))
        )
    }

    #[test]
    fn parse_miss_comma_or_right_square_bracket_works() {
        assert_eq!(
            Parser::new("[1").parse(),
            Err(JsonError::MissCommaOrRightSquareBracket)
        );
        assert_eq!(
            Parser::new("[1}").parse(),
            Err(JsonError::MissCommaOrRightSquareBracket)
        );
        assert_eq!(
            Parser::new("[1 \"abc\"").parse(),
            Err(JsonError::MissCommaOrRightSquareBracket)
        );
        assert_eq!(
            Parser::new("[1, []").parse(),
            Err(JsonError::MissCommaOrRightSquareBracket)
        );
    }

    #[test]
    fn parse_object_works() {
        if let Ok(JsonValue::Object(map)) = Parser::new(
            r#" { 
                "n" : null , 
                "f" : false , 
                "t" : true , 
                "i" : 123 , 
                "s" : "abc", 
                "a" : [ 1, 2, 3 ],
                "o" : { "1" : 1, "2" : 2, "3" : 3 }
                 } "#,
        )
        .parse()
        {
            println!("{:?}", map);
            assert_eq!(
                map,
                HashMap::from([
                    ("n".into(), JsonValue::Null),
                    ("f".into(), JsonValue::Bool(false)),
                    ("t".into(), JsonValue::Bool(true)),
                    ("i".into(), JsonValue::Number(123f64)),
                    ("s".into(), JsonValue::String("abc".into())),
                    (
                        "a".into(),
                        JsonValue::Array(vec![
                            JsonValue::Number(1f64),
                            JsonValue::Number(2f64),
                            JsonValue::Number(3f64)
                        ])
                    ),
                    (
                        "o".into(),
                        JsonValue::Object(HashMap::from([
                            ("1".into(), JsonValue::Number(1f64)),
                            ("2".into(), JsonValue::Number(2f64)),
                            ("3".into(), JsonValue::Number(3f64))
                        ]))
                    )
                ])
            )
        }
    }
}
