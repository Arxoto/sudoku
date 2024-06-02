#![allow(dead_code)]

use super::{
    entity::{is_sudoku_value, new_sudoku_matrix, SudokuMatrix, SudokuValueType, SQUARE_OUTER_LEN},
    rulers::get_sudoku_ruler_loop,
};

pub struct Map {
    value: SudokuMatrix<SudokuValueType>,
}

#[derive(Debug, PartialEq)]
pub struct ProbabilyMap {
    value: SudokuMatrix<SudokuValueType>,
}

impl Map {}

impl Into<ProbabilyMap> for Map {
    fn into(self) -> ProbabilyMap {
        let sudoku_loop = get_sudoku_ruler_loop();

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
            value: new_sudoku_matrix(0),
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
    use crate::sudoku::rulers::init;

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

        init();
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

        init();
        let pmap: ProbabilyMap = map.into();
        for ll in pmap.value.iter() {
            println!("{:?}", ll);
        }

        println!("{:?}", pmap.find_most_probabily());
    }
}
