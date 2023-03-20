use crate::bitboard::{self, Bitboard};
use crate::position::Position;
use crate::square::Square;

pub const ROOK_DELTAS: [i32; 4] = [8, 1, -8, -1];
pub const BISHOP_DELTAS: [i32; 4] = [9, 7, -9, -7];
pub const KING_DELTAS: [i32; 8] = [9, 8, 7, 1, -9, -8, -7, -1];
pub const KNIGHT_DELTAS: [i32; 8] = [17, 15, 10, 6, -17, -15, -10, -6];
pub const WHITE_PAWN_DELTAS: [i32; 2] = [7, 9];
pub const BLACK_PAWN_DELTAS: [i32; 2] = [-7, -9];

pub fn sliding_attacks(square: Square, occupied: Bitboard, deltas: &[i32]) -> Bitboard {
    let mut attacks = Bitboard::new(0);
    for delta in deltas {
        let mut previous = square;

        while let Some(square) = previous.offset(*delta) {
            if square.distance(previous) > 2 {
                break;
            }
            attacks.add(square);
            if occupied.contains(square) {
                break;
            }
            previous = square;
        }
    }
    attacks
}

/// Calculate the piece mask without the endpoints of the ray.
/// Only relevant for sliding pieces.
pub fn magic_mask(square: Square, deltas: &[i32]) -> Bitboard {
    let origin = square;
    let mut mask = bitboard::EMPTY;
    for delta in deltas {
        let mut previous = square;
        while let Some(square) = previous.offset(*delta) {
            if square.distance(previous) != 1 {
                break;
            }
            mask.add(previous);
            previous = square;
        }
    }
    mask.remove(origin);
    mask
}
