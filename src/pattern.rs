use std::fmt::Write;
use std::str::FromStr;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    #[error("trying to parse an empty pattern")]
    Empty,
    #[error("pattern contains only wildcards")]
    MissingByte,
    #[error("pattern is malformed")]
    Malformed,
}

#[derive(Debug, Clone, Copy)]
enum PatternByte {
    Wildcard,
    Byte(u8),
}

impl PatternByte {
    fn is_wildcard(self) -> bool {
        matches!(self, PatternByte::Wildcard)
    }
    fn as_byte(self) -> Option<u8> {
        match self {
            PatternByte::Wildcard => None,
            PatternByte::Byte(byte) => Some(byte),
        }
    }
    fn matches(self, byte: u8) -> bool {
        match self {
            PatternByte::Wildcard => true,
            PatternByte::Byte(b) => b == byte,
        }
    }
}

#[derive(Debug)]
pub struct Pattern {
    inner: Vec<PatternByte>,
}

impl Pattern {
    fn matches(pattern: &[PatternByte], hay: &[u8]) -> bool {
        if pattern.len() > hay.len() {
            false
        } else {
            pattern.iter().zip(hay.iter()).all(|(&p, &b)| p.matches(b))
        }
    }

    pub fn find_in(&self, hay: &[u8]) -> Option<usize> {
        let skipped_wildcards = self.inner.iter().position(|p| !p.is_wildcard())?;

        let hay = &hay[skipped_wildcards..];
        let pattern = &self.inner[skipped_wildcards..];

        // SAFETY: The pattern must contain at least one non-wildcard
        let first_byte = pattern[0].as_byte().unwrap();

        let mut offset = 0usize;
        while let Some(sub_offset) = memchr::memchr(first_byte, &hay[offset..]) {
            if Self::matches(pattern, &hay[offset + sub_offset..]) {
                return Some(offset + sub_offset);
            }
            offset += sub_offset + 1;
        }
        None
    }
}

impl FromStr for Pattern {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_hex(ascii: u8) -> Option<u8> {
            match ascii {
                b'A'..=b'F' => Some(ascii - b'A' + 0xA),
                b'0'..=b'9' => Some(ascii - b'0'),
                _ => None,
            }
        }

        fn parse_token(input: &mut &[u8]) -> Result<PatternByte, Error> {
            if !input.is_empty() && (*input)[0] == b'?' {
                *input = &(*input)[1..];
                Ok(PatternByte::Wildcard)
            } else if input.len() > 1 {
                let byte = parse_hex((*input)[0]).ok_or(Error::Malformed)? * 0x10
                    + parse_hex((*input)[1]).ok_or(Error::Malformed)?;
                *input = &(*input)[2..];
                Ok(PatternByte::Byte(byte))
            } else {
                Err(Error::Malformed)
            }
        }

        fn parse_whitespace(input: &mut &[u8]) -> Result<(), Error> {
            if !input.is_empty() && (*input)[0] == b' ' {
                *input = &(*input)[1..];
                Ok(())
            } else {
                Err(Error::Malformed)
            }
        }

        let mut input = s.as_bytes();
        let mut buffer = Vec::new();

        if !input.is_empty() {
            buffer.push(parse_token(&mut input)?);
            while !input.is_empty() {
                parse_whitespace(&mut input)?;
                buffer.push(parse_token(&mut input)?);
            }
        } else {
            return Err(Error::Empty);
        }

        if buffer.iter().all(|&b| b.is_wildcard()) {
            return Err(Error::MissingByte);
        }

        Ok(Pattern { inner: buffer })
    }
}

impl std::fmt::Display for PatternByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternByte::Wildcard => f.write_char('?'),
            PatternByte::Byte(b) => write!(f, "{:02X}", b),
        }
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut segments = self.inner.iter();
        if let Some(segment) = segments.next() {
            write!(f, "{}", segment)?;
            for segment in segments {
                f.write_char(' ')?;
                write!(f, "{}", segment)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_pattern {
    use super::*;

    #[test]
    fn parses() {
        let pattern: Pattern = "0F FF ? ? A1".parse().unwrap();
        let pattern_str = pattern.to_string();
        assert_eq!(pattern_str, "0F FF ? ? A1");
    }

    #[test]
    fn errors() {
        // only wildcards make no sense
        let err = Pattern::from_str("? ? ?").unwrap_err();
        assert_eq!(err, Error::MissingByte);

        // empty pattern makes no sense
        let err = Pattern::from_str("").unwrap_err();
        assert_eq!(err, Error::Empty);

        // nice byte
        let err = Pattern::from_str("ZZ").unwrap_err();
        assert_eq!(err, Error::Malformed);

        // double space is not cool
        let err = Pattern::from_str("00  00").unwrap_err();
        assert_eq!(err, Error::Malformed);
    }

    #[test]
    fn finds() {
        let pattern: Pattern = "? ? 01 ? 03".parse().unwrap();
        let hay = &[0x00, 0x01, 0x01, 0x02, 0x03];
        assert_eq!(pattern.find_in(hay), Some(0));
    }
}
