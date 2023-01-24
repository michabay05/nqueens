use crate::GET_SQ;

// BB is essentially an extension to the 'u64' datatype

pub type BB = u64;

// Methods added to the 'BB' type
pub trait BBTrait {
    fn print(num: u64);
    fn get_bit(&self, ind: i8) -> bool;
    fn set_bit(&mut self, ind: i8);
    fn pop_lsb(&mut self) -> i8;
}

impl BBTrait for BB {
    fn print(num: BB) {
        // Prints the bitboard in the form of an 8x8 board
        for r in 0..8 {
            print!(" {} |", 8 - r);
            for f in 0..8 {
                print!(" {}", if num.get_bit(GET_SQ!(r, f)) { "1" } else { "." });
            }
            println!();
        }
        println!("     - - - - - - - -");
        println!("     a b c d e f g h\n");
    }

    /* UTIL METHODS */
    fn get_bit(&self, ind: i8) -> bool {
        // If the bit on the given index is on, return true
        if *self & (1 << ind) != 0 {
            return true;
        }
        false
    }

    fn set_bit(&mut self, ind: i8) {
        // Turn the bit on the given index on
        *self |= 1 << ind;
    }

    fn pop_lsb(&mut self) -> i8 {
        // Get the least significant bit from the bitboard,
        // store its index and turn the bit on that index off
        if *self > 0 {
            let result = *self ^ (*self - 1);
            let lsb_index = (result.count_ones() as i8) - 1;
            *self &= !(1 << lsb_index);
            return lsb_index;
        }
        // If the bitboard is equal to zero, return 0
        0
    }
}
