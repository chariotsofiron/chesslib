// /// https://www.rhysre.net/fast-chess-move-generation-with-magic-bitboards.html
// /// https://peterellisjones.com/posts/generating-legal-chess-moves-efficiently/
// /// https://www.chessprogramming.org/index.php?title=Looking_for_Magics
// ///
// /// https://www.chessprogramming.org/Best_Magics_so_far
// /// https://github.com/GunshipPenguin/shallow-blue/blob/c6d7e9615514a86533a9e0ffddfc96e058fc9cfd/src/attacks.h#L120
// /// https://www.codeproject.com/Articles/5313417/Worlds-fastest-Bitboard-Chess-Movegenerator

mod action;
mod bitboard;
mod color;
mod piece;
mod position;
mod square;
mod util;
use bitboard::Bitboard;
use bitintr::Popcnt;
use square::Square;
use std::fmt::LowerHex;
use std::io::Write;
use util::{magic_mask, sliding_attacks};

fn generate_tables<W: Write>(f: &mut W) -> std::io::Result<()> {
    let mut rook_masks = vec![bitboard::EMPTY; 64];
    let mut bishop_masks = vec![bitboard::EMPTY; 64];
    let mut king_masks = vec![bitboard::EMPTY; 64];
    let mut knight_masks = vec![bitboard::EMPTY; 64];

    for square in 0..64 {
        let square = Square::new(square);
        rook_masks[usize::from(square)] =
            sliding_attacks(square, bitboard::EMPTY, &util::ROOK_DELTAS);
        bishop_masks[usize::from(square)] =
            sliding_attacks(square, bitboard::EMPTY, &util::BISHOP_DELTAS);
        king_masks[usize::from(square)] =
            sliding_attacks(square, bitboard::FULL, &util::KING_DELTAS);
        knight_masks[usize::from(square)] =
            sliding_attacks(square, bitboard::FULL, &util::KNIGHT_DELTAS);
    }

    dump_slice(f, "ROOK_MASKS", "u64", &rook_masks)?;
    dump_slice(f, "BISHOP_MASKS", "u64", &rook_masks)?;
    dump_slice(f, "KING_MASKS", "u64", &rook_masks)?;
    dump_slice(f, "KNIGHT_MASKS", "u64", &rook_masks)?;

    let (magics, offsets): (Vec<Bitboard>, [usize; 64]) = generate_magics(&util::ROOK_DELTAS);
    dump_slice(f, "ROOK_MAGICS", "u64", &magics)?;
    dump_slice(f, "ROOK_OFFSETS", "usize", &offsets)?;

    let (magics, offsets): (Vec<Bitboard>, [usize; 64]) = generate_magics(&util::BISHOP_DELTAS);
    dump_slice(f, "BISHOP_MAGICS", "u64", &magics)?;
    dump_slice(f, "BISHOP_OFFSETS", "usize", &offsets)?;

    Ok(())
}

/// generate magics for sliding pieces
/// given the deltas, returns bitboards and square-offsets
fn generate_magics(deltas: &[i32]) -> (Vec<Bitboard>, [usize; 64]) {
    let mut magics: Vec<Bitboard> = Vec::new();

    let mut offset = 0;
    let mut offsets: [usize; 64] = [0; 64];

    for sq in 0..64 {
        let square = Square::new(sq);
        let magic_mask = magic_mask(square, deltas);
        for subset in magic_mask.carry_rippler() {
            let attack = sliding_attacks(square, subset, deltas);
            magics.push(attack);
        }
        offsets[usize::from(square)] = offset;
        offset += (1 << u64::from(magic_mask).popcnt()) as usize;
    }
    (magics, offsets)
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path = std::path::Path::new(&out_dir).join("constants.rs");
    let mut f = std::fs::File::create(&path).unwrap();
    generate_tables(&mut f).unwrap();
}

fn dump_slice<W: Write, T: Clone + LowerHex>(
    w: &mut W,
    name: &str,
    tname: &str,
    slice: &[T],
) -> std::io::Result<()> {
    writeln!(w, "#[allow(clippy::unreadable_literal)]").unwrap();
    writeln!(w, "pub const {}: [{}; {}] = [", name, tname, slice.len())?;
    for v in slice.iter().cloned() {
        writeln!(w, "    {:#018x},", v)?;
    }
    writeln!(w, "];")
}
