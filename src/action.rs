use crate::piece::{Piece, PieceType};
use crate::square::{ParseError, Square};

/// https://www.chessprogramming.org/Encoding_Moves
/// Module is called action due to reserved keywords :D
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
}


impl Move {
    pub fn new(from: Square, to: Square, promotion: Option<PieceType>) -> Move {
        Move {
            from,
            to,
            promotion,
        }
    }

    pub fn from_uci(uci: &str) -> Result<Move, ParseError> {
        let from = Square::try_from(&uci[0..2])?;
        let to = Square::try_from(&uci[2..4])?;
        let promotion = if uci.len() > 4 {
            Some(PieceType::try_from(&uci[4..5])?)
        } else {
            None
        };
        Ok(Move::new(from, to, promotion))
    }
}
