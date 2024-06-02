use super::{
    entity::{is_sudoku_value, new_sudoku_matrix_value, SudokuMatrixValue, SQUARE_OUTER_LEN},
    rulers::get_sudoku_loop,
};

// https://sudoku.com/zh/shu-du-gui-ze/
// https://www.conceptispuzzles.com/zh/index.aspx?uri=puzzle/sudoku/techniques
// 基于排除法的技巧：对有所的可能性进行枚举，并根据三大规则排除
// 1、某一位置 剩余唯一候选数值  -- 可确定值
// 2、某一分区 某一数值 仅有一个位置可选  -- 可确定值
// 3、某一分区 某一数值 多个位置可选 所有位置均在另一分区  -- 可将另一分区的其他位置标记该值的黑名单
// 4、某一分区 多个数值 多个位置可选 数值和位置是互斥关系  -- 可将这些位置的其他候选值标记黑名单

pub struct Map {
    value: SudokuMatrixValue,
}

#[derive(Debug, PartialEq)]
pub struct ProbabilyMap {
    value: SudokuMatrixValue,
}

impl Map {}

impl Into<ProbabilyMap> for Map {
    fn into(self) -> ProbabilyMap {
        let sudoku_loop = get_sudoku_loop();

        let mut pmap = ProbabilyMap::new();
        for ruler in sudoku_loop.iter() {
            for ll in ruler.partitions.iter() {
                let mut count = 0;
                for (x, y) in ll.iter() {
                    if is_sudoku_value(self.value[*x][*y]) {
                        count += 1;
                    }
                }
                for (x, y) in ll.iter() {
                    if !is_sudoku_value(self.value[*x][*y]) {
                        pmap.value[*x][*y] += count;
                    }
                }
            }
        }
        pmap
    }
}

impl ProbabilyMap {
    pub fn new() -> ProbabilyMap {
        ProbabilyMap {
            value: new_sudoku_matrix_value(),
        }
    }

    pub fn find_most_probabily(self) -> (usize, usize) {
        let mut max = 0;
        let mut result = (0, 0);
        for row in 0..SQUARE_OUTER_LEN {
            for col in 0..SQUARE_OUTER_LEN {
                if self.value[row][col] > max {
                    max = self.value[row][col];
                    result = (row, col);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::rulers::init_sudoku_loop;

    use super::*;

    #[test]
    fn test_probabily() {
        // let map = Map {
        //     value: [
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //         [0, 0, 0, 0, 0, 0, 0, 0, 0],
        //     ],
        // };
        let map = Map {
            value: [
                [0, 0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 1, 1, 0, 0, 0, 0],
                [1, 1, 1, 1, 1, 1, 1, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0],
            ],
        };

        init_sudoku_loop();
        let pmap: ProbabilyMap = map.into();
        assert_eq!(
            pmap,
            ProbabilyMap {
                value: [
                    [2, 2, 2, 6, 0, 5, 2, 1, 1],
                    [2, 2, 2, 6, 0, 5, 2, 1, 1],
                    [2, 2, 2, 6, 0, 5, 2, 1, 1],
                    [5, 5, 5, 9, 0, 8, 3, 2, 2],
                    [6, 6, 6, 0, 0, 9, 4, 3, 3],
                    [0, 0, 0, 0, 0, 0, 0, 8, 8],
                    [2, 2, 2, 6, 0, 5, 2, 1, 1],
                    [2, 2, 2, 6, 0, 5, 2, 1, 1],
                    [2, 2, 2, 6, 0, 5, 2, 1, 1],
                ]
            }
        );

        let (x, y) = pmap.find_most_probabily();
        assert_eq!(x, 3);
        assert_eq!(y, 3);
    }

    #[test]
    fn test_xx() {
        let map = Map {
            value: [
                [0, 0, 0, 1, 0, 4, 0, 0, 0],
                [0, 0, 1, 0, 0, 0, 9, 0, 0],
                [0, 9, 0, 7, 0, 3, 0, 6, 0],
                [8, 0, 7, 0, 0, 0, 1, 0, 6],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [3, 0, 4, 0, 0, 0, 5, 0, 9],
                [0, 5, 0, 4, 0, 2, 0, 3, 0],
                [0, 0, 8, 0, 0, 0, 6, 0, 0],
                [0, 0, 0, 8, 0, 6, 0, 0, 0],
            ],
        };

        init_sudoku_loop();
        let pmap: ProbabilyMap = map.into();
        for ll in pmap.value.iter() {
            println!("{:?}", ll);
        }

        println!("{:?}", pmap.find_most_probabily());
    }
}
