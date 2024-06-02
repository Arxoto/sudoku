use super::{
    entity::{is_sudoku_value, new_sudoku_matrix_value, SudokuMatrixValue, SQUARE_OUTER_LEN},
    rulers::get_sudoku_loop,
};

// https://sudoku.com/zh/shu-du-gui-ze/
// https://www.conceptispuzzles.com/zh/index.aspx?uri=puzzle/sudoku/techniques
// 基础的技巧：
// 1、候选排除法，对有所的可能性进行枚举，并根据三大规则排除，剩下唯一值
//    -- 程序实现简单，算法复杂度高，实际运用困难，后期才有使用价值，但候选可能性算法应该是后面所有算法的基础
// 2、选值排除法，先选定一个已知数量最多的值，根据行列规则排除，剩下在九宫格内有唯一位置
//    -- 实际应用最常用的技巧，上述例子中行列黑名单、九宫格白名单，未验证过黑白名单的规则互换后是否等效（感觉不等效）
// 3、连带推理法，基于上面技巧，若九宫格内剩余多个位置，但其均在一行一列，则可将该行该列的其余位置标志黑名单，再次运用上面技巧
// 4、可选互斥法，

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
