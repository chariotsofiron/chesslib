use crate::square::Square;
use derive_more;
use std::fmt;

pub const EMPTY: Bitboard = Bitboard(0);
pub const FULL: Bitboard = Bitboard(!0);

/// Uses little-endian rank-file bitboard representation.
/// square = <rank, file>
/// https://www.chessprogramming.org/Square_Mapping_Considerations#LittleEndianRankFileMapping
#[derive(
    derive_more::BitOrAssign,
    derive_more::BitOr,
    derive_more::Not,
    derive_more::BitAnd,
    derive_more::BitAndAssign,
    Clone,
    Copy,
    derive_more::LowerHex,
    derive_more::Into,
)]
pub struct Bitboard(u64);

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::with_capacity(64 * 2);
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Square::new(rank << 3 | file);
                if self.contains(square) {
                    s.push_str("X ");
                } else {
                    s.push_str(". ");
                }
            }
            s.push_str("\n")
        }
        write!(f, "{}", s)
    }
}

impl Bitboard {
    /// Construct a new Bitboard from a u64
    #[inline]
    pub fn new(b: u64) -> Bitboard {
        Bitboard(b)
    }

    #[inline]
    pub fn from_square(sq: Square) -> Bitboard {
        Bitboard(1 << u64::from(sq))
    }

    #[inline]
    pub fn add<T: Into<Bitboard>>(&mut self, squares: T) {
        *self |= squares.into();
    }

    #[inline]
    pub fn remove<T: Into<Bitboard>>(&mut self, squares: T) {
        *self &= !squares.into();
    }

    #[inline]
    pub fn contains(self, sq: Square) -> bool {
        !(self & Bitboard::from_square(sq)).is_empty()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// An iterator over the subsets of this bitboard.
    #[inline]
    pub fn carry_rippler(self) -> CarryRippler {
        CarryRippler {
            bb: self.0,
            subset: 0,
            first: true,
        }
    }
}

/// Iterator over the subsets of a [`Bitboard`].
/// https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
#[derive(Debug, Clone)]
pub struct CarryRippler {
    bb: u64,
    subset: u64,
    first: bool,
}

impl Iterator for CarryRippler {
    type Item = Bitboard;

    #[inline]
    fn next(&mut self) -> Option<Bitboard> {
        let subset = self.subset;
        if subset != 0 || self.first {
            self.first = false;
            self.subset = self.subset.wrapping_sub(self.bb) & self.bb;
            Some(Bitboard(subset))
        } else {
            None
        }
    }
}

impl From<Square> for Bitboard {
    fn from(sq: Square) -> Bitboard {
        Bitboard::from_square(sq)
    }
}