use crate::action::Move;
use crate::bitboard::{self, Bitboard};
use crate::color::Color;
use crate::piece::{Piece, PieceType};
use crate::square::Square;
use enum_map::{enum_map, EnumMap};

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Position {
    white_mask: Bitboard,
    black_mask: Bitboard,
    all_mask: Bitboard,

    // TODO use this insetad https://doc.rust-lang.org/std/ops/trait.Index.html
    bitboards: EnumMap<PieceType, Bitboard>,
    en_passant: Option<Square>,
    castling_rights: u8,
    player_move: Color,
}

impl Position {
    pub fn new() -> Position {
        Position {
            white_mask: Bitboard::new(0xffff),
            black_mask: Bitboard::new(0xffff_0000_0000_0000),
            all_mask: Bitboard::new(0xffff_0000_0000_ffff),
            bitboards: enum_map! {
                PieceType::Pawn => Bitboard::new(0x00ff_0000_0000_ff00),
                PieceType::Knight => Bitboard::new(0x4200_0000_0000_0042),
                PieceType::Bishop => Bitboard::new(0x2400_0000_0000_0024),
                PieceType::Rook => Bitboard::new(0x8100_0000_0000_0081),
                PieceType::Queen => Bitboard::new(0x0800_0000_0000_0008),
                PieceType::King => Bitboard::new(0x1000_0000_0000_0010),
            },
            en_passant: None,
            castling_rights: 0,
            player_move: Color::White,
        }
    }

    pub fn from_fen(fen: &str) -> Position {
        let mut position = Position::new();

        let rows = fen.split('/');


        position
    }

    fn remove_piece_at(&mut self, square: Square) {
        self.bitboards[PieceType::Pawn].remove(square);
        self.bitboards[PieceType::Knight].remove(square);
        self.bitboards[PieceType::Bishop].remove(square);
        self.bitboards[PieceType::Rook].remove(square);
        self.bitboards[PieceType::Queen].remove(square);
        self.black_mask.remove(square);
        self.white_mask.remove(square);
        self.all_mask.remove(square);
    }

    fn set_piece_at(&mut self, square: Square, piece: Piece) {
        self.remove_piece_at(square);
        self.bitboards[piece.piece_type].add(square);
        self.black_mask.add(square);
        self.white_mask.add(square);
        self.all_mask.add(square);
    }

    pub fn push_uci(&mut self, uci: &str) {
        let Move {from, to, promotion} = Move::from_uci(uci).unwrap();
        let piece = self.piece_at(from).unwrap();
        if let Some(promotion) = promotion {
            self.set_piece_at(to, Piece::new(promotion, piece.color));
        } else {
            self.set_piece_at(to, piece);
        }
        self.remove_piece_at(from);
    }

    #[inline]
    pub fn piece_type_at(&self, sq: Square) -> Option<PieceType> {
        if !self.all_mask.contains(sq) {
            None // catch early
        } else if self.bitboards[PieceType::Pawn].contains(sq) {
            Some(PieceType::Pawn)
        } else if self.bitboards[PieceType::Knight].contains(sq) {
            Some(PieceType::Knight)
        } else if self.bitboards[PieceType::Bishop].contains(sq) {
            Some(PieceType::Bishop)
        } else if self.bitboards[PieceType::Rook].contains(sq) {
            Some(PieceType::Rook)
        } else if self.bitboards[PieceType::Queen].contains(sq) {
            Some(PieceType::Queen)
        } else {
            Some(PieceType::King)
        }
    }

    #[inline]
    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        self.piece_type_at(sq).map(|piece_type| {
            Piece::new(
                piece_type,
                match self.white_mask.contains(sq) {
                    true => Color::White,
                    false => Color::Black,
                },
            )
        })
    }

    /// Gets the color of the piece at the given square.
    pub fn color_at(&self, sq: Square) -> Option<Color> {
        if self.white_mask.contains(sq) {
            Some(Color::White)
        } else if self.black_mask.contains(sq) {
            Some(Color::Black)
        } else {
            None
        }
    }

    pub fn legal_moves(&self)  {
        unimplemented!()
    }

    pub fn is_insufficient_material(&self) -> bool {
        unimplemented!()
    }

    pub fn is_stalemate(&self) -> bool {
        unimplemented!()
    }

    pub fn is_check(&self) -> bool {
        unimplemented!()
    }

    pub fn get_moves() -> Vec<Move> {
        unimplemented!()
    }

    pub fn get_pseudo_legal_moves() -> Vec<Move> {
        unimplemented!()
    }

    pub fn zobrist_hash() -> u64 {
        unimplemented!()
    }
}

// /// Recalculate rook_pin and bishop_pin for the given position.
// pub fn refresh(position: Position, kingban: Bitboard, check_mask: Bitboard) -> Bitboard {
//     let rook_pin = bitboard::EMPTY;
//     let bishop_pin = bitboard::EMPTY;

//     attacks::get_bishop_moves(king_square, occupied);
//     // bishop attacks

//     bitboard::EMPTY
// }


impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board: String = String::with_capacity(64 * 2);
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Square::new(rank << 3 | file);

                if let Some(piece) = self.piece_at(square) {
                    board.push_str(&piece.symbol().to_string());
                } else {
                    board.push_str(".");
                }
            }
            board.push_str("\n")
        }
        write!(f, "{}", board)
    }
}