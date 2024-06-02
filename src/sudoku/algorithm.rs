//! https://sudoku.com/zh/shu-du-gui-ze/
//! https://www.conceptispuzzles.com/zh/index.aspx?uri=puzzle/sudoku/techniques
//! 基于排除法的技巧：对有所的可能性进行枚举，并根据三大规则排除
//! 1、某一位置 剩余唯一候选数值  -- 可确定值
//! 2、某一分区 某一数值 仅有一个位置可选  -- 可确定值
//! 3、某一分区 某一数值 多个位置可选 所有位置均在另一分区  -- 可将另一分区的其他位置标记该值的黑名单
//! 4、某一分区 多个数值 多个位置可选 数值和位置是互斥关系  -- 可将这些位置的其他候选值标记黑名单

use super::{
    entity::{
        is_sudoku_value, new_sudoku_matrix, SudokuMatrix, SudokuMatrixValue, SudokuValueType,
        SQUARE_OUTER_LEN,
    },
    rulers::{get_sudoku_ruler_partition_map, Position},
};

#[derive(Copy, Clone)]
pub struct Candidate {
    pub can: [bool; SQUARE_OUTER_LEN],
}

impl Candidate {
    pub fn new_all() -> Candidate {
        Candidate {
            can: [true; SQUARE_OUTER_LEN],
        }
    }
    pub fn new_none() -> Candidate {
        Candidate {
            can: [false; SQUARE_OUTER_LEN],
        }
    }

    pub fn only(&self) -> Option<SudokuValueType> {
        let mut count = 0;
        let mut some = 0;
        for (i, ele) in self.can.iter().enumerate() {
            if *ele {
                count += 1;
                some = i;
            }
        }
        return if count == 1 { Some(some + 1) } else { None };
    }
}

#[derive(Clone, Copy)]
pub struct CandidateMatrix {
    can_matrix: SudokuMatrix<Candidate>,
}

impl CandidateMatrix {
    pub fn new() -> CandidateMatrix {
        CandidateMatrix {
            can_matrix: new_sudoku_matrix(Candidate::new_all()),
        }
    }

    fn set_partition_black_list(&mut self, value: &SudokuValueType, pos: &Position) {
        let partition_list = get_sudoku_ruler_partition_map(pos);
        for ll in partition_list.iter() {
            for (row, col) in ll.iter() {
                self.can_matrix[*row][*col].can[value - 1] = false;
            }
        }
    }

    pub fn evolution(&mut self) {
        let shadow = self.clone();
        for (row, ll) in shadow.can_matrix.iter().enumerate() {
            for (col, value) in ll.iter().enumerate() {
                if let Some(value) = value.only() {
                    let pos = (row, col);
                    self.set_partition_black_list(&value, &pos);
                    self.can_matrix[row][col].can[value - 1] = true;
                }
            }
        }
    }
}

impl Into<SudokuMatrixValue> for CandidateMatrix {
    fn into(self) -> SudokuMatrixValue {
        let mut result = SudokuMatrixValue::new();

        for (row, ll) in self.can_matrix.iter().enumerate() {
            for (col, can) in ll.iter().enumerate() {
                if let Some(value) = can.only() {
                    result.matrix[row][col] = value;
                }
            }
        }

        result
    }
}

impl Into<CandidateMatrix> for SudokuMatrixValue {
    fn into(self) -> CandidateMatrix {
        let mut result = CandidateMatrix::new();

        for (row, ll) in self.matrix.iter().enumerate() {
            for (col, value) in ll.iter().enumerate() {
                if is_sudoku_value(*value) {
                    result.can_matrix[row][col] = Candidate::new_none();
                    result.can_matrix[row][col].can[value - 1] = true;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::rulers::init;

    use super::*;

    #[test]
    fn test_into_candidate_and_evolution() {
        init();

        let sudoku: SudokuMatrixValue = SudokuMatrixValue {
            matrix: [
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 2, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 3, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        };
        let can: CandidateMatrix = sudoku.into();
        assert_eq!(can.can_matrix[0][0].can, [true; 9]);
        assert_eq!(
            can.can_matrix[1][1].can,
            [true, false, false, false, false, false, false, false, false]
        );
        assert_eq!(
            can.can_matrix[2][2].can,
            [false, true, false, false, false, false, false, false, false]
        );
        assert_eq!(
            can.can_matrix[3][3].can,
            [false, false, true, false, false, false, false, false, false]
        );

        // 演进
        let mut can = can;
        can.evolution();
        assert_eq!(can.can_matrix[8][8].can, [true; 9]);

        // 1 2 在九宫格内
        assert_eq!(
            can.can_matrix[0][0].can,
            [false, false, true, true, true, true, true, true, true]
        );

        // 行列规则
        assert_eq!(
            can.can_matrix[1][8].can,
            [false, true, true, true, true, true, true, true, true]
        );
        assert_eq!(
            can.can_matrix[8][2].can,
            [true, false, true, true, true, true, true, true, true]
        );

        // 3上面位置 与2同行 与3九宫格
        assert_eq!(
            can.can_matrix[2][3].can,
            [true, false, false, true, true, true, true, true, true]
        );
    }

    #[test]
    fn test_into() {
        init();

        let sudoku: SudokuMatrixValue = SudokuMatrixValue {
            matrix: [
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 2, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 3, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        };

        let can: CandidateMatrix = sudoku.into();
        let next_sudoku: SudokuMatrixValue = can.into();

        assert_eq!(sudoku, next_sudoku);
    }

    #[test]
    fn test_evolution_and_into() {
        init();

        let sudoku: SudokuMatrixValue = SudokuMatrixValue {
            matrix: [
                [1, 0, 3, 4, 5, 6, 7, 8, 9],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        };

        let mut can: CandidateMatrix = sudoku.into();
        can.evolution();
        // for ll in can.can_matrix.iter() {
        //     for l in ll.iter() {
        //         println!("{:?}", l.can);
        //     }
        // }
        let next_sudoku: SudokuMatrixValue = can.into();
        assert_eq!(
            next_sudoku,
            SudokuMatrixValue {
                matrix: [
                    [1, 2, 3, 4, 5, 6, 7, 8, 9],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                ],
            }
        );
    }
}
