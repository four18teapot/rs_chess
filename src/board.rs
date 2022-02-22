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
v10-   "   Queen
11-   "   King
12- Empty
13- Offboard
*/

pub type Square = usize;
pub type Piece = u8;

pub const BOARD64: [Square; 64] =
    [21, 22, 23, 24, 25, 26, 27, 28,
     31, 32, 33, 34, 35, 36, 37, 38,
     41, 42, 43, 44, 45, 46, 47, 48,
     51, 52, 53, 54, 55, 56, 57, 58,
     61, 62, 63, 64, 65, 66, 67, 68,
     71, 72, 73, 74, 75, 76, 77, 78,
     81, 82, 83, 84, 85, 86, 87, 88,
     91, 92, 93, 94, 95, 96, 97, 98];

pub const BOARD120: [Square; 120] =
    [usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
     usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
     usize::MAX,  0,  1,  2,  3,  4,  5,  6,  7, usize::MAX,
     usize::MAX,  8,  9, 10, 11, 12, 13, 14, 15, usize::MAX,
     usize::MAX, 16, 17, 18, 19, 20, 21, 22, 23, usize::MAX,
     usize::MAX, 24, 25, 26, 27, 28, 29, 30, 31, usize::MAX,
     usize::MAX, 32, 33, 34, 35, 36, 37, 38, 39, usize::MAX,
     usize::MAX, 40, 41, 42, 43, 44, 45, 46, 47, usize::MAX,
     usize::MAX, 48, 49, 50, 51, 52, 53, 54, 55, usize::MAX,
     usize::MAX, 56, 57, 58, 59, 60, 61, 62, 63, usize::MAX,
     usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX,
     usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX];
    

pub const BOARD_START: [Piece; 120] = 
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

pub const BOARD_EMPTY: [Piece; 120] =
    [13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
     13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
     13,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     13,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     13,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     13,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     13,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     13,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     13,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     13,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
     13, 13, 13, 13, 13, 13, 13, 13, 13, 13];
     

const KNIGHT_OFFSETS: [i64; 8] = [-19, -21, -12, 8, 19, 21, -8, 12];

/// Represents a chess gamestate, along with usefull functions and fields for move generation and state evaluation.
#[derive(Copy, Clone)]
pub struct Board {
    /// Representation of the actual chess board in a [12x10 Mailbox System](https://www.chessprogramming.org/10x12_Board).
    board: [Piece; 120],
    /// Array of the castling permitions ([WhiteKing, WhiteQueen, BlackKing, BlackQueen]).
    castle: [bool; 4],
    /// The side to make a move. Equal to 1 if white and -1 if black.
    side: i8,
    /// Number of half-moves allready played.
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
    /// A standard chess starting position.
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

    /// Board struct from [Forsyth-Edwards-Notation](https://de.wikipedia.org/wiki/Forsyth-Edwards-Notation).
    pub fn from_fen(fen: &str) -> Option<Board> {
	let fen_byte = fen.as_bytes();

	
	let mut board = BOARD_EMPTY;
	let mut piece_count = [0; 12];
	let mut piece_pos = [[0; 10]; 12];
	let mut en_passant: Option<Square> = None;
	let mut side = 0;
	let mut moves = 0;
	let mut fifty_moves = 0;
	let mut key_history = [0; 2048];
	let mut move_history = [Move::NoMove; 2048];
	let mut castle = [false; 4];

	let mut square: Square = 0;
	let mut byte_idx = 0;
	loop {
	    if square == 64 {
		break;
	    } else if square > 64 {
		return None;
	    }

	    let token = match fen_byte.get(byte_idx) {
		None => return None,
		Some(a) => a,
	    };
	    
	    if *token == b'/' {
		if square % 8 != 7 {
		    return None;
		}
	    } else if *token > 47 && *token < 58 {
		square += (token - 48) as usize;
	    } else {
		let piece = match fen_byte.get(byte_idx) {
		    Some(b'P') => 0,
		    Some(b'R') => 1,
		    Some(b'N') => 2,
		    Some(b'B') => 3,
		    Some(b'Q') => 4,
		    Some(b'K') => 5,
		    Some(b'p') => 6,
		    Some(b'r') => 7,
		    Some(b'n') => 8,
		    Some(b'b') => 9,
		    Some(b'q') => 10,
		    Some(b'k') => 11,
		    _ => return None,
		};
		let real_sq = BOARD64[square];
		piece_pos[piece][piece_count[piece]] = real_sq;
		piece_count[piece] += 1;
		square += 1;
	    }
	    byte_idx += 1;
	}

	byte_idx += 1;
	side = match fen_byte.get(byte_idx) {
	    Some(b'w') => 1,
	    Some(b'b') => -1,
	    _ => return None,
	};

	byte_idx += 1;
	loop {
	    castle[match fen_byte.get(byte_idx) {
		Some(b'K') => 0,
		Some(b'Q') => 1,
		Some(b'k') => 2,
		Some(b'q') => 3,
		Some(b' ') => break,
		_ => return None,
	    }] = true;
	    byte_idx += 1;
	}

	byte_idx += 1;
	todo!();
    }

    pub fn pseudo_legal_moves(&self) -> Vec<Move> {
	let mut moves: Vec<Move> = Vec::new();
	todo!();
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
	    _  => unreachable!(),
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
    /// (from, to)
    Normal(Square, Square),
    /// (from, to)
    DoublePawn(Square, Square),
    /// (from, to) 
    EnPassant(Square, Square),
    /// (from, to, piece)
    Promotion(Square, Square, u8),
    /// (type of castle) [0 - WhiteKing, 1 - WhiteQueen, 2 - BlackKing, 3 - BlackQueen] 
    Castle(usize),
}

/// Return the type of a piece. (1 = White, -1 = Black, 0 = Empty, 2 = Offboard)
fn piece_type(piece: Piece) -> i8 {
    return if piece < 6 { 1 } else if piece < 12 { -1 } else if piece == 12 { 0 } else { 2 };
}

fn algebraic_to_square(alg: &str) -> Option<Square> {
    todo!();
}
