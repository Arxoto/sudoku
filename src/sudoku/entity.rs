pub type SudokuValueType = usize;
pub const SUDOKU_UNKNOWN: SudokuValueType = 0;

pub fn is_sudoku_value(value: SudokuValueType) -> bool {
    match value {
        1..=9 => true,
        _ => false,
    }
}

pub const MATRIX_INNER_LEN: usize = 3;
pub const MATRIX_INNER_COUNT: usize = 3;
pub const MATRIX_LEN: usize = 9;
pub type NineNineMatrix = [[SudokuValueType; MATRIX_LEN]; MATRIX_LEN];
pub fn new_nine_nine_matrix() -> NineNineMatrix {
    [[SUDOKU_UNKNOWN; MATRIX_LEN]; MATRIX_LEN]
}
