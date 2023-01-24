use crate::GET_SQ;

pub type BB = u64;

pub trait BBTrait {
    fn print(num: u64);
    fn get_bit(&self, sq: i8) -> bool;
    fn set_bit(&mut self, sq: i8);
    fn pop_lsb(&mut self) -> i8;
}

impl BBTrait for BB {
    fn print(num: BB) {
        for r in 0..8 {
            print!(" {} |", 8 - r);
            for f in 0..8 {
                print!(" {}", if num.get_bit(GET_SQ!(r, f)) { "1" } else { "." });
            }
            println!();
        }
        println!("     - - - - - - - -");
        println!("     a b c d e f g h\n");

        println!("     Decimal: {}", num);
        println!(" Hexadecimal: {:#x}", num);
    }

    fn get_bit(&self, sq: i8) -> bool {
        if *self & (1 << sq) != 0 {
            return true;
        }
        false
    }

    fn set_bit(&mut self, sq: i8) {
        *self |= 1 << sq;
    }

    fn pop_lsb(&mut self) -> i8 {
        if *self > 0 {
            let result = *self ^ (*self - 1);
            let lsb_index = (result.count_ones() as i8) - 1;
            *self &= !(1 << lsb_index);
            return lsb_index;
        }
        0
    }
}
