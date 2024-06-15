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
    rulers::{each_sudoku_partition, get_sudoku_ruler_partition_map, Position, RULER_COUNT},
};

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
pub struct CandidateMatrix {
    pub can_matrix: SudokuMatrix<Candidate>,
}

impl CandidateMatrix {
    pub fn new() -> CandidateMatrix {
        CandidateMatrix {
            can_matrix: new_sudoku_matrix(Candidate::new_all()),
        }
    }

    pub fn finished(&self) -> bool {
        let mut finished = true;
        for ll in self.can_matrix.iter() {
            for can in ll.iter() {
                finished &= can.only().is_some();
            }
        }
        finished
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
            for (col, can) in ll.iter().enumerate() {
                if let Some(value) = can.only() {
                    let pos = (row, col);
                    self.set_partition_black_list(&value, &pos);
                    self.can_matrix[row][col].can[value - 1] = true;
                }
            }
        }
    }

    pub fn evolution_by_check_position(&mut self) {
        each_sudoku_partition(|ruler_id, partition| {
            for value_id in 0..SQUARE_OUTER_LEN {
                let mut count = 0;
                let mut pos = [(0, 0); RULER_COUNT];
                for (row, col) in partition.iter() {
                    if self.can_matrix[*row][*col].can[value_id] {
                        count += 1;
                        if count > RULER_COUNT {
                            break;
                        }
                        pos[count - 1] = (*row, *col);
                    }
                }
                let count = count;
                let pos = pos;
                match count {
                    1 => {
                        // 仅一个位置可选 值可确定
                        let (row, col) = pos[0];
                        self.can_matrix[row][col] = Candidate::new_none();
                        self.can_matrix[row][col].can[value_id] = true;
                    }
                    // 多个位置可选
                    2 => {
                        let partition_map = get_sudoku_ruler_partition_map(&pos[0]);
                        for (current_ruler_id, partition) in partition_map.iter().enumerate() {
                            if ruler_id == current_ruler_id {
                                continue;
                            }
                            if partition.contains(&pos[1]) {
                                // 所有位置均在某一分区 可排除该分区其他位置
                                for pp in partition.iter() {
                                    if *pp != pos[0] && *pp != pos[1] {
                                        self.can_matrix[pp.0][pp.1].can[value_id] = false;
                                    }
                                }
                                break;
                            }
                        }
                    }
                    3 => {
                        let partition_map = get_sudoku_ruler_partition_map(&pos[0]);
                        for (current_ruler_id, partition) in partition_map.iter().enumerate() {
                            if ruler_id == current_ruler_id {
                                continue;
                            }
                            if partition.contains(&pos[1]) && partition.contains(&pos[2]) {
                                // 所有位置均在某一分区 可排除该分区其他位置
                                for pp in partition.iter() {
                                    if *pp != pos[0] && *pp != pos[1] && *pp != pos[2] {
                                        self.can_matrix[pp.0][pp.1].can[value_id] = false;
                                    }
                                }
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    pub fn evolution_by_position_mutex(&mut self) {
        each_sudoku_partition(|_, partition| {
            // value_id -> position_id -> (row, col, is_candidate)
            let mut candidate_map = [[(0, 0, false); SQUARE_OUTER_LEN]; SQUARE_OUTER_LEN];
            for value_id in 0..SQUARE_OUTER_LEN {
                for (pos_id, (row, col)) in partition.iter().enumerate() {
                    candidate_map[value_id][pos_id] =
                        (*row, *col, self.can_matrix[*row][*col].can[value_id]);
                }
            }
            // 两两互斥
            let mut double_map = [false; SQUARE_OUTER_LEN];
            for (value_id, position_ids) in candidate_map.iter().enumerate() {
                double_map[value_id] = position_ids.iter().filter(|p| p.2).count() == 2;
            }
            // 找到位置互斥的元素
            let double_map = double_map;
            for (value_id, position_ids) in candidate_map.iter().enumerate() {
                if !double_map[value_id] {
                    continue;
                }
                for second_value_id in (value_id + 1)..SQUARE_OUTER_LEN {
                    if !double_map[second_value_id] {
                        continue;
                    }
                    if *position_ids == candidate_map[second_value_id] {
                        for (row, col, is_candidate) in position_ids.iter() {
                            if *is_candidate {
                                self.can_matrix[*row][*col] = Candidate::new_none();
                                self.can_matrix[*row][*col].can[value_id] = true;
                                self.can_matrix[*row][*col].can[second_value_id] = true;
                            }
                        }
                    }
                }
            }
            // 仨仨互斥
            let mut triple_map = [false; SQUARE_OUTER_LEN];
            for (value_id, position_ids) in candidate_map.iter().enumerate() {
                triple_map[value_id] = position_ids.iter().filter(|p| p.2).count() == 3;
            }
            // 找到位置互斥的元素
            let triple_map = triple_map;
            for first_value_id in 0..SQUARE_OUTER_LEN {
                if !triple_map[first_value_id] {
                    continue;
                }
                for second_value_id in (first_value_id + 1)..SQUARE_OUTER_LEN {
                    if !triple_map[second_value_id] {
                        continue;
                    }
                    for third_value_id in (second_value_id + 1)..SQUARE_OUTER_LEN {
                        if !triple_map[third_value_id] {
                            continue;
                        }
                        let position_ids = candidate_map[first_value_id];
                        if position_ids == candidate_map[second_value_id]
                            && position_ids == candidate_map[third_value_id]
                        {
                            for (row, col, is_candidate) in position_ids.iter() {
                                if *is_candidate {
                                    self.can_matrix[*row][*col] = Candidate::new_none();
                                    self.can_matrix[*row][*col].can[first_value_id] = true;
                                    self.can_matrix[*row][*col].can[second_value_id] = true;
                                    self.can_matrix[*row][*col].can[third_value_id] = true;
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

impl From<CandidateMatrix> for SudokuMatrixValue {
    fn from(value: CandidateMatrix) -> Self {
        let mut target = SudokuMatrixValue::new();

        for (row, ll) in value.can_matrix.iter().enumerate() {
            for (col, can) in ll.iter().enumerate() {
                if let Some(value) = can.only() {
                    target.matrix[row][col] = value;
                }
            }
        }

        target
    }
}
impl From<SudokuMatrixValue> for CandidateMatrix {
    fn from(value: SudokuMatrixValue) -> Self {
        let mut target = CandidateMatrix::new();

        for (row, ll) in value.matrix.iter().enumerate() {
            for (col, value) in ll.iter().enumerate() {
                if is_sudoku_value(*value) {
                    target.can_matrix[row][col] = Candidate::new_none();
                    target.can_matrix[row][col].can[value - 1] = true;
                }
            }
        }

        target
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

    #[test]
    fn test_only_one_position() {
        init();

        let sudoku: SudokuMatrixValue = SudokuMatrixValue {
            matrix: [
                [0, 0, 0, 0, 0, 6, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 6],
                [1, 2, 0, 0, 0, 0, 0, 0, 0],
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
        can.evolution_by_check_position();
        let next_sudoku: SudokuMatrixValue = can.into();
        assert_eq!(
            next_sudoku,
            SudokuMatrixValue {
                matrix: [
                    [0, 0, 0, 0, 0, 6, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 6],
                    [1, 2, 6, 0, 0, 0, 0, 0, 0],
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

    #[test]
    fn test_much_position_in_same_other_partition() {
        init();

        let sudoku: SudokuMatrixValue = SudokuMatrixValue {
            matrix: [
                [0, 0, 0, 0, 0, 6, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 2, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 6, 0, 0, 0, 0, 0, 0],
            ],
        };
        let mut can: CandidateMatrix = sudoku.into();
        can.evolution();
        can.evolution_by_check_position();
        assert_eq!(
            can.can_matrix[1][6].can,
            [true, true, true, true, true, false, true, true, true]
        );
        assert_eq!(
            can.can_matrix[1][7].can,
            [true, true, true, true, true, false, true, true, true]
        );
        assert_eq!(
            can.can_matrix[1][8].can,
            [true, true, true, true, true, false, true, true, true]
        );
    }

    #[test]
    fn test_position_double_mutex() {
        init();

        let sudoku: SudokuMatrixValue = SudokuMatrixValue {
            matrix: [
                [0, 0, 0, 0, 0, 0, 0, 1, 2],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 6, 0, 0, 0, 0, 0, 0, 0],
                [2, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [6, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 2, 0, 0, 0, 0, 0, 0, 0],
            ],
        };
        let mut can: CandidateMatrix = sudoku.into();
        can.evolution();
        can.evolution_by_position_mutex();
        can.evolution_by_check_position();
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
                    [0, 0, 6, 0, 0, 0, 0, 1, 2],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 6, 0, 0, 0, 0, 0, 0, 0],
                    [2, 1, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [6, 0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 2, 0, 0, 0, 0, 0, 0, 0],
                ],
            }
        );
    }

    #[test]
    fn test_position_triple_mutex() {
        init();

        let sudoku: SudokuMatrixValue = SudokuMatrixValue {
            matrix: [
                [0, 0, 0, 0, 0, 0, 0, 0, 6],
                [0, 0, 0, 0, 0, 6, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [3, 0, 0, 0, 0, 0, 0, 0, 0],
                [2, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [6, 3, 0, 0, 0, 0, 0, 0, 0],
                [1, 2, 0, 0, 0, 0, 0, 0, 0],
            ],
        };
        let mut can: CandidateMatrix = sudoku.into();
        can.evolution();
        can.evolution_by_position_mutex();
        can.evolution_by_check_position();
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
                    [0, 0, 0, 0, 0, 0, 0, 0, 6],
                    [0, 0, 0, 0, 0, 6, 0, 0, 0],
                    [0, 6, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [3, 0, 0, 0, 0, 0, 0, 0, 0],
                    [2, 1, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [6, 3, 0, 0, 0, 0, 0, 0, 0],
                    [1, 2, 0, 0, 0, 0, 0, 0, 0],
                ],
            }
        );
    }
}
