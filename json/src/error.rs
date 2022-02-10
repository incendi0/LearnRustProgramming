use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum JsonError {
    UnexpectedValue,
    RootNotSingular,
    InvalidNumber,
    MissEndQuotationMark,
    InvalidUnicodeSurrogate,
    InvalidUnicodeHex,
    InvalidStringEscape,
    InvalidStringChar,
    MissCommaOrRightSquareBracket,
    MissEntryKey,
    MissEntryColon,
    MissCommaOrRightCurlyBracket,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            JsonError::UnexpectedValue => write!(f, "unexpected value"),
            JsonError::RootNotSingular => write!(f, "root not singular"),
            JsonError::InvalidNumber => write!(f, "invalid number"),
            JsonError::MissEndQuotationMark => write!(f, "miss end quotation mark"),
            JsonError::InvalidUnicodeSurrogate => write!(f, "invalid unicode surrogate"),
            JsonError::InvalidUnicodeHex => write!(f, "invalid unicode hex"),
            JsonError::InvalidStringEscape => write!(f, "invalid string escape"),
            JsonError::InvalidStringChar => write!(f, "invalid string char"),
            JsonError::MissCommaOrRightSquareBracket => {
                write!(f, "miss comma or right square bracket")
            }
            JsonError::MissEntryKey => write!(f, "miss entry key for object"),
            JsonError::MissEntryColon => write!(f, "miss entry colon for object"),
            JsonError::MissCommaOrRightCurlyBracket => {
                write!(f, "miss comma or right curly bracket")
            }
        }
    }
}

impl error::Error for JsonError {}
