mod board;
mod hasher;

use board::Board;

fn main() {
    let mut b = Board::new_starting();
    print!("{}", b);
}
