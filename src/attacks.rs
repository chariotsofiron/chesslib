use crate::bitboard::Bitboard;
use crate::square::Square;
use bitintr::Pext;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub fn get_rook_moves(square: Square, occupied: Bitboard) -> Bitboard {
    let piece_mask = ROOK_MASKS[usize::from(square)];
    let index = u64::from(occupied).pext(piece_mask) as usize;
    let offset = ROOK_OFFSETS[usize::from(square)];
    Bitboard::new(ROOK_MAGICS[offset + index])
}

pub fn get_bishop_moves(square: Square, occupied: Bitboard) -> Bitboard {
    let piece_mask = BISHOP_MASKS[usize::from(square)];
    let index = u64::from(occupied).pext(piece_mask) as usize;
    let offset = BISHOP_OFFSETS[usize::from(square)];
    Bitboard::new(BISHOP_MAGICS[offset + index])
}
