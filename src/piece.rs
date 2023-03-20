use enum_map::Enum;

use crate::square::ParseError;
use crate::color::Color;

#[derive(Enum)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl TryFrom<&str> for PieceType {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_str() {
            "p" => Ok(PieceType::Pawn),
            "n" => Ok(PieceType::Knight),
            "b" => Ok(PieceType::Bishop),
            "r" => Ok(PieceType::Rook),
            "q" => Ok(PieceType::Queen),
            "k" => Ok(PieceType::King),
            _ => Err(ParseError),
        }
    }
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    pub fn symbol(&self) -> char {
        let letter = match self.piece_type {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        if self.color == Color::White {
            letter.to_ascii_uppercase()
        } else {
            letter
        }
    }
}
