use crate::bb::{BBTrait, BB};
use crate::{GET_COL, GET_ROW, GET_SQ};

// A structure that stores all the possible moves for a queen
// Since a queen is essentially a combination of a bishop and rook,
// the rook and bishop moves are stored
pub struct SlidingAttack {
    pub bishop_moves: [BB; 64],
    pub rook_moves: [BB; 64],
}

impl SlidingAttack {
    pub fn new() -> Self {
        let mut instance = Self {
            bishop_moves: [0; 64],
            rook_moves: [0; 64],
        };
        instance.init();
        instance
    }

    fn init(&mut self) {
        // Initializes the possible moves for both bishop and rook
        // on all 64 squares
        for sq in 0..64 {
            self.bishop_moves[sq] = gen_bishop_attack(sq as u8);
            self.rook_moves[sq] = gen_rook_attack(sq as u8);
        }
    }
}

fn gen_bishop_attack(sq: u8) -> BB {
    // Create and return a bitboard representation of all
    // the possible moves for a bishop on a given square 
    
    // Store the soon-to-be-returned bitboard representation
    let mut bishop_moves: BB = 0;

    // Store the starting row and col as reference
    let start_row: i8 = GET_ROW!(sq);
    let start_col: i8 = GET_COL!(sq);

    // Modifiable row and col counter
    let mut r: i8 = start_row;
    let mut c: i8 = start_col;

    // North East direction
    r = start_row + 1;
    c = start_col + 1;
    while r <= 7 && c <= 7 {
        bishop_moves.set_bit(GET_SQ!(r, c));

        r += 1;
        c += 1;
    }

    // North West direction
    r = start_row + 1;
    c = start_col - 1;
    while r <= 7 && c >= 0 {
        bishop_moves.set_bit(GET_SQ!(r, c));

        r += 1;
        c -= 1;
    }

    // South East direction
    r = start_row - 1;
    c = start_col + 1;
    while r >= 0 && c <= 7 {
        bishop_moves.set_bit(GET_SQ!(r, c));

        r -= 1;
        c += 1;
    }

    // South West direction
    r = start_row - 1;
    c = start_col - 1;
    while r >= 0 && c >= 0 {
        bishop_moves.set_bit(GET_SQ!(r, c));

        r -= 1;
        c -= 1;
    }
    bishop_moves
}

fn gen_rook_attack(sq: u8) -> BB {
    // Create and return a bitboard representation of all
    // the possible moves for a rook on a given square 

    // Store the soon-to-be-returned bitboard representation
    let mut rook_moves: BB = 0;

    // Store the starting row and col as reference
    let start_row: i8 = GET_ROW!(sq);
    let start_col: i8 = GET_COL!(sq);

    // Modifiable row and col counter
    let mut r: i8 = start_row;
    let mut c: i8 = start_col;

    // North direction
    r = start_row + 1;
    while r <= 7 {
        rook_moves.set_bit(GET_SQ!(r, start_col));

        r += 1;
    }

    // South direction
    r = start_row - 1;
    while r >= 0 {
        rook_moves.set_bit(GET_SQ!(r, start_col));

        r -= 1;
    }

    // East direction
    c = start_col + 1;
    while c <= 7 {
        rook_moves.set_bit(GET_SQ!(start_row, c));
        c += 1;
    }

    // West direction
    c = start_col - 1;
    while c >= 0 {
        rook_moves.set_bit(GET_SQ!(start_row, c));

        c -= 1;
    }
    rook_moves
}
