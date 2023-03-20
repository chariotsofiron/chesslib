mod action;
mod attacks;
mod bitboard;
mod color;
mod piece;
mod position;
mod square;
mod util;


#[cfg(test)]
mod tests {
    use crate::square::Square;
    use crate::position::Position;

    #[test]
    fn it_works() {

        let square = Square::try_from("e2").unwrap();

        let mut x = Position::new();
        println!("{}", x);
        
        x.push_uci("e2e4");
        println!("{}", x);
    }
}
