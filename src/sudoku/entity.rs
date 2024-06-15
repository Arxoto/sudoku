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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SudokuMatrixValue {
    pub matrix: SudokuMatrix<SudokuValueType>
}

impl SudokuMatrixValue {
    pub fn new() -> SudokuMatrixValue {
        SudokuMatrixValue { matrix: new_sudoku_matrix(SUDOKU_UNKNOWN) }
    }

    pub fn next_empty_value(&self) -> Option<(usize, usize)> {
        for i in 0..SQUARE_OUTER_LEN {
            for j in 0..SQUARE_OUTER_LEN {
                if !is_sudoku_value(self.matrix[i][j]) {
                    return Some((i, j));
                }
            }
        }
        None
    }
}
