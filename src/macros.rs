#[macro_export]
macro_rules! GET_SQ {
    ($row: expr, $col: expr) => {
        ($row * 8) + $col
    };
}

#[macro_export]
macro_rules! GET_ROW {
    ($sq: expr) => {
        ($sq as i8 >> 3i8)
    };
}

#[macro_export]
macro_rules! GET_COL {
    ($sq: expr) => {
        ($sq as i8 & 7i8)
    };
}

#[macro_export]
macro_rules! SQ_STR {
    ($row: expr, $col: expr) => {{
        let row_char = ((((8 - $row) as u8) % ('0' as u8)) + ('0' as u8));
        let file_char = (($col as u8) % ('a' as u8)) + ('a' as u8);
        let sq_str = format!("{}{}", file_char as char, row_char as char);
        sq_str
    }};
}
