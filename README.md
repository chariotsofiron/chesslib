Need Zen3 arch for best performance.

## Variants
- Standard
- Suicide
- Giveaway
- Antichess
- Atomic
- King of the Hill
- Racing Kings
- Horde
- Three-Check
- Crazyhouse


```rust
use chesslib::Game;

let game = Game::new();
board.push_san("e4")?;
board.push_san("e5")?;
board.push_san("Qh5")?;
board.push_san("Nc6")?;
board.push_san("Bc4")?;
board.push_san("Nf6")?;
board.push_san("Qxf7")?;
board.is_checkmate();
/// true

board.fen();
/// Board('r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4')

```