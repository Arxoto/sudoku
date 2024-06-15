use super::{
    algorithm::CandidateMatrix,
    entity::{is_sudoku_value, SudokuMatrixValue, SudokuValueType, SQUARE_OUTER_LEN, SUDOKU_UNKNOWN},
    rulers::{get_sudoku_ruler_partition_map, Position},
};

fn is_valid(matrix: &SudokuMatrixValue, pos: &Position, num: SudokuValueType) -> bool {
    if !is_sudoku_value(num) {
        return false;
    }

    let mut valided = true;
    let partition_list = get_sudoku_ruler_partition_map(pos);
    for partition in partition_list {
        for (row, col) in partition {
            valided &= num != matrix.matrix[row][col];
        }
    }
    valided
}

pub struct SudokuSolver {
    candi: CandidateMatrix,
    current: SudokuMatrixValue,
    all_possible: Vec<SudokuMatrixValue>,
}

impl From<CandidateMatrix> for SudokuSolver {
    fn from(value: CandidateMatrix) -> Self {
        let origin = SudokuMatrixValue::from(value);
        return SudokuSolver {
            candi: value,
            current: origin,
            all_possible: Vec::new(),
        };
    }
}

impl SudokuSolver {
    pub fn solver_possible(&mut self) {
        if let Some((row, col)) = self.current.next_empty_value() {
            let cans = self.candi.can_matrix[row][col].can;
            for num in 0..SQUARE_OUTER_LEN {
                if cans[num] && is_valid(&self.current, &(row, col), num + 1) {
                    self.current.matrix[row][col] = num + 1;
                    self.solver_possible();
                }
            }
            self.current.matrix[row][col] = SUDOKU_UNKNOWN;
        } else {
            self.all_possible.push(self.current);
        }
    }

    pub fn get_all_possible_sudoku(&self) -> &Vec<SudokuMatrixValue> {
        &self.all_possible
    }
}
