use rand::prelude::*;
use crate::board::*;

type HashKey = u64;

pub struct Hasher {
    table: [[HashKey; 64]; 12],
}

impl Hasher {
    fn empty() -> Hasher {
	Hasher {
	    table: [[0; 64]; 12],
	}
    }

    fn from_seed() -> Hasher {
	let mut table = [[0; 64]; 12];
	for piece in 0..12 {
	    for sq in 0..64 {
		table[piece][sq] = rand::random::<HashKey>();
	    }
	}
	Hasher {
	    table: table,
	}
    }

    pub fn hash_board(&self, b: &Board) -> HashKey {
	let mut key: HashKey = 0;
	for sq in BOARD64 {
	    let piece = b.board[sq];
	    if piece > 11 {
		continue
	    }

	    key ^= self.table[piece as usize][BOARD120[sq]];
	}
	return key;
    }

    pub fn key_at(&self, p: Piece, sq: Square) -> HashKey {
	self.table[p as usize][BOARD120[sq]]
    }
}
