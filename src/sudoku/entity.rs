pub const SQUARE_OUTER_LEN: usize = 9;
pub const SQUARE_INNER_LEN: usize = 3;
pub const SQUARE_INNER_NUM: usize = SQUARE_OUTER_LEN / SQUARE_INNER_LEN;

pub type SudokuMatrix<T> = [[T; SQUARE_OUTER_LEN]; SQUARE_OUTER_LEN];
pub fn new_sudoku_matrix<T: Copy>(init_value: T) -> SudokuMatrix<T> {
    [[init_value; SQUARE_OUTER_LEN]; SQUARE_OUTER_LEN]
}

pub type SudokuValueType = usize;
pub const SUDOKU_UNKNOWN: SudokuValueType = 0;
pub fn is_sudoku_value(value: SudokuValueType) -> bool {
    match value {
        1..=9 => true,
        _ => false,
    }
}

pub type SudokuMatrixValue = SudokuMatrix<SudokuValueType>;
pub fn new_sudoku_matrix_value() -> SudokuMatrixValue {
    new_sudoku_matrix(SUDOKU_UNKNOWN)
}
