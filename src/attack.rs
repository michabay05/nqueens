use crate::bb::{BBTrait, BB};
use crate::{GET_COL, GET_ROW, GET_SQ};

pub struct SlidingAttack {
    pub bishop_occupancy: [BB; 64],
    pub rook_occupancy: [BB; 64],
}

impl SlidingAttack {
    pub fn new() -> Self {
        let mut instance = Self {
            bishop_occupancy: [0; 64],
            rook_occupancy: [0; 64],
        };
        instance.init(true);
        instance.init(false);
        instance
    }

    fn init(&mut self, is_bishop: bool) {
        for sq in 0..64 {
            self.bishop_occupancy[sq] = gen_bishop_attack(sq as u8);
            self.rook_occupancy[sq] = gen_rook_attack(sq as u8);
        }
    }
}

fn gen_bishop_attack(sq: u8) -> BB {
    let mut bishop_moves: BB = 0;

    let SOURCE_ROW: i8 = GET_ROW!(sq);
    let SOURCE_COL: i8 = GET_COL!(sq);
    let mut r: i8 = SOURCE_ROW;
    let mut c: i8 = SOURCE_COL;

    r = SOURCE_ROW + 1;
    c = SOURCE_COL + 1;
    while r <= 7 && c <= 7 {
        bishop_moves.set_bit(GET_SQ!(r, c));

        r += 1;
        c += 1;
    }

    r = SOURCE_ROW + 1;
    c = SOURCE_COL - 1;
    while r <= 7 && c >= 0 {
        bishop_moves.set_bit(GET_SQ!(r, c));

        r += 1;
        c -= 1;
    }

    r = SOURCE_ROW - 1;
    c = SOURCE_COL + 1;
    while r >= 0 && c <= 7 {
        bishop_moves.set_bit(GET_SQ!(r, c));

        r -= 1;
        c += 1;
    }

    r = SOURCE_ROW - 1;
    c = SOURCE_COL - 1;
    while r >= 0 && c >= 0 {
        bishop_moves.set_bit(GET_SQ!(r, c));

        r -= 1;
        c -= 1;
    }
    bishop_moves
}

fn gen_rook_attack(sq: u8) -> BB {
    let mut rook_moves: BB = 0;

    let SOURCE_ROW: i8 = GET_ROW!(sq);
    let SOURCE_COL: i8 = GET_COL!(sq);
    let mut r: i8 = SOURCE_ROW;
    let mut c: i8 = SOURCE_COL;

    r = SOURCE_ROW + 1;
    while r <= 7 {
        rook_moves.set_bit(GET_SQ!(r, SOURCE_COL));

        r += 1;
    }

    r = SOURCE_ROW - 1;
    while r >= 0 {
        rook_moves.set_bit(GET_SQ!(r, SOURCE_COL));

        r -= 1;
    }

    c = SOURCE_COL + 1;
    while c <= 7 {
        rook_moves.set_bit(GET_SQ!(SOURCE_ROW, c));

        c += 1;
    }

    c = SOURCE_COL - 1;
    while c >= 0 {
        rook_moves.set_bit(GET_SQ!(SOURCE_ROW, c));

        c -= 1;
    }
    rook_moves
}
