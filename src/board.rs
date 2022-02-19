use std::{fmt};

/*
Pieces are represented by a u8.
0 - White Pawn
1 -   "   Rook
2 -   "   Knight
3 -   "   Bishop
4 -   "   Queen
5 -   "   King
6 - Black Pawn
7 -   "   Rook
8 -   "   Knight
9 -   "   Bishop
10-   "   Queen
11-   "   King
12- Empty
13- Offboard
*/

type Square = usize;
type Piece = u8;

const BOARD64: [Square; 64] =
    [21, 22, 23, 24, 25, 26, 27, 28,
     31, 32, 33, 34, 35, 36, 37, 38,
     41, 42, 43, 44, 45, 46, 47, 48,
     51, 52, 53, 54, 55, 56, 57, 58,
     61, 62, 63, 64, 65, 66, 67, 68,
     71, 72, 73, 74, 75, 76, 77, 78,
     81, 82, 83, 84, 85, 86, 87, 88,
     91, 92, 93, 94, 95, 96, 97, 98];

const BOARD_START: [Piece; 120] = 
    [13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
     13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
     13,  7,  8,  9, 10, 11,  9,  8,  7, 13,
     13,  6,  6,  6,  6,  6,  6,  6,  6, 13,
     13, 12, 12, 12, 12, 12, 12, 12, 12, 13,
     13, 12, 12, 12, 12, 12, 12, 12, 12, 13,
     13, 12, 12, 12, 12, 12, 12, 12, 12, 13,
     13, 12, 12, 12, 12, 12, 12, 12, 12, 13,
     13,  0,  0,  0,  0,  0,  0,  0,  0, 13,
     13,  1,  2,  3,  4,  5,  3,  2,  1, 13,
     13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
     13, 13, 13, 13, 13, 13, 13, 13, 13, 13];

/// Represents a chess gamestate, along with usefull functions and fields for move generation and state evaluation.
#[derive(Copy, Clone)]
pub struct Board {
    /// Representation of the actual chess board in a [12x10 Mailbox System](https://www.chessprogramming.org/10x12_Board).
    board: [Piece; 120],
    /// Array of the castling permitions ([WhiteKing, WhiteQueen, BlackKing, BlackQueen]).
    castle: [bool; 4],
    /// The side to make a move. Equal to 1 if white and -1 if black.
    side: i8,
333    /// Number of half-moves allready played.
    moves: u16,
    /// Array of the history of [Zorbist Hashes](https://en.wikipedia.org/wiki/Zobrist_hashing) the game has been in. Used for efficient evaluation and detecting three-fold repitition.
    key_history: [u64; 2048],
    /// History of [`Move`]s allready played.
    move_history: [Move; 2048],
    /// Counter for the fifty move rule.
    fifty_moves: u16,
    /// Some(n) -> the square a enPassant capture could appear on. None -> no enPassant capture can be made.
    en_passant: Option<Square>,
    /// Count of how many pieces exist for each piece.
    piece_count: [u8; 12],
    /// List of all the positions of all pieces existing on the board.
    piece_pos: [[Square; 10]; 12],
}

impl Board {
    pub fn new_starting() -> Board {
	Board {
	    board: BOARD_START,
	    castle: [true, true, true, true],
	    side: 1,
	    moves: 0,
	    key_history: [0; 2048],
	    move_history: [Move::NoMove; 2048],
	    fifty_moves: 0,
	    en_passant: None,
	    piece_count: [8, 2, 2, 2, 1, 1, 8, 2, 2, 2, 1, 1],
	    piece_pos: [[81, 82, 83, 84, 85, 86, 87, 88, 0, 0],
	                [91, 98, 0, 0, 0, 0, 0, 0, 0, 0],
	                [92, 97, 0, 0, 0, 0, 0, 0, 0, 0],
	                [93, 96, 0, 0, 0, 0, 0, 0, 0, 0],
	                [94,  0, 0, 0, 0, 0, 0, 0, 0, 0],
	                [95,  0, 0, 0, 0, 0, 0, 0, 0, 0],
	                [31, 32, 33, 34, 35, 36, 37, 38, 0, 0],
			[21, 28, 0, 0, 0, 0, 0, 0, 0, 0],
			[22, 27, 0, 0, 0, 0, 0, 0, 0, 0],
			[23, 26, 0, 0, 0, 0, 0, 0, 0, 0],
			[24,  0, 0, 0, 0, 0, 0, 0, 0, 0],
			[25,  0, 0, 0, 0, 0, 0, 0, 0, 0]],		
	}
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	let mut str_board = String::new();
	for sq in BOARD64 {
	    str_board.push_str(match self.board[sq] {
		0 => "P",
		1 => "R",
		2 => "N",
		3 => "B",
		4 => "Q",
		5 => "K",
		6 => "p",
		7 => "r",
		8 => "n",
		9 => "b",
		10 => "q",
		11 => "k",
		12 => "#",
		_ => unreachable!(),
	    });
	    if sq % 10 == 8 {
		str_board.push_str("\n")
	    }
	}

	str_board.push_str("\n");

	str_board.push_str(match self.side {
	    1  => "Side: White\n",
	    -1 => "Side: Black\n",
3	    _  => unreachable!(),
	});

	let str_mov_count = format!("Move Count: {}\n", self.moves);
	str_board.push_str(&str_mov_count);
	
	write!(f, "{}", str_board)
    }
}

/// Represents all the diffrent moves that can be applied to [`Board`].
#[derive(Clone, Copy)]
enum Move {
    /// Is not a move, used for storing moves in arrays.
    NoMove,
    /// (from, to, is_capture)
    Normal(Square, Square, bool),
    /// (from, to)
    DoublePawn(Square, Square),
    /// (from, to) 
    EnPassant(Square, Square),
    /// (from, to, piece, is_capture)
    Promotion(Square, Square, u8, bool),
    /// (type of castle) [0 - WhiteKing, 1 - WhiteQueen, 2 - BlackKing, 3 - BlackQueen] 
    Castle(usize),
}