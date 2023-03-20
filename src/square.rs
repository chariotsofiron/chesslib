use std::cmp::max;
use std::fmt;


#[derive(Clone, Debug)]
pub struct ParseError;
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid square")
    }
}


const NSQUARES: u8 = 64;

fn abs_diff(lhs: u8, rhs: u8) -> u8 {
    if lhs < rhs {
        rhs - lhs
    } else {
        lhs - rhs
    }
}

#[derive(Copy, Clone, derive_more::Into)]
#[into(types(usize, u64))]
pub struct Square(u8);

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (('a' as u8) + ((self.0 & 7) as u8)) as char,
            (('1' as u8) + ((self.0 >> 3) as u8)) as char
        )
    }
}

impl TryFrom<&str> for Square {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Square, ParseError> {
        if s.len() != 2 {
            return Err(ParseError);
        }
        let ch: Vec<char> = s.chars().map(|c| c.to_ascii_lowercase()).collect();
        let rank = (ch[0] as u8).checked_sub('a' as u8).ok_or(ParseError)?;
        let file = (ch[1] as u8).checked_sub('1' as u8).ok_or(ParseError)?;

        if rank > 7 || file > 7 {
            return Err(ParseError);
        }
        return Ok(Square(rank | file << 3));
    }
}

/// 8 bit integer representing rank and file
impl Square {
    pub fn new(index: u8) -> Square {
        Square(index % NSQUARES)
    }

    pub fn offset(&self, delta: i32) -> Option<Square> {
        let value = self.0 as i32 + delta;
        match value {
            0..=63 => Some(Square(value as u8)),
            _ => None,
        }
    }

    /// Number of king moves to get from one square to another.
    pub fn distance(&self, other: Square) -> u8 {
        max(
            abs_diff(self.0 & 7, other.0 & 7),
            abs_diff(self.0 >> 3, other.0 >> 3),
        )
    }
}
