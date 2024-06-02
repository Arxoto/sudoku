use super::entity::{SQUARE_INNER_LEN, SQUARE_INNER_NUM, SQUARE_OUTER_LEN};

/// 元素的位置
pub type Position = (usize, usize);
/// 划分后同一系列元素
pub type PositionPartition = [Position; SQUARE_OUTER_LEN];
/// 根据规则进行不同划分 行以行分 列以列分
#[derive(Copy, Clone)]
pub struct SudokuRuler {
    pub partitions: [PositionPartition; SQUARE_OUTER_LEN],
}

/// 三大规则：行、列、九宫格内数字不重复
pub type SudokuRulerLoop = [SudokuRuler; 3];
fn gen_sudoku_loop() -> SudokuRulerLoop {
    let mut sudoku_loop: SudokuRulerLoop = [SudokuRuler {
        partitions: [[(0, 0); SQUARE_OUTER_LEN]; SQUARE_OUTER_LEN],
    }; 3];

    // row
    for row in 0..SQUARE_OUTER_LEN {
        for col in 0..SQUARE_OUTER_LEN {
            sudoku_loop[0].partitions[row][col] = (row, col);
        }
    }

    // column
    for col in 0..SQUARE_OUTER_LEN {
        for row in 0..SQUARE_OUTER_LEN {
            sudoku_loop[1].partitions[col][row] = (row, col);
        }
    }

    // matrix
    let mut each_num = 0;
    for row_m in 0..SQUARE_INNER_NUM {
        for col_m in 0..SQUARE_INNER_NUM {
            let row_start = row_m * SQUARE_INNER_LEN;
            let row_final = row_m * SQUARE_INNER_LEN + SQUARE_INNER_LEN;
            let col_start = col_m * SQUARE_INNER_LEN;
            let col_final = col_m * SQUARE_INNER_LEN + SQUARE_INNER_LEN;

            let mut element_num = 0;
            for row in row_start..row_final {
                for col in col_start..col_final {
                    sudoku_loop[2].partitions[each_num][element_num] = (row, col);
                    element_num += 1;
                }
            }

            each_num += 1;
        }
    }

    sudoku_loop
}

static mut RULER_LOOP: Option<SudokuRulerLoop> = None;

pub fn init_sudoku_loop() {
    unsafe {
        RULER_LOOP = Some(gen_sudoku_loop());
    }
}

pub fn get_sudoku_loop() -> SudokuRulerLoop {
    unsafe { RULER_LOOP.unwrap() }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::entity::new_sudoku_matrix_value;

    use super::*;

    #[test]
    fn test() {
        let sudoku_loop = gen_sudoku_loop();

        println!("========= >>>>>> row <<<<<< =========");
        let row_ruler = sudoku_loop[0];
        for l in row_ruler.partitions.iter() {
            println!("{:?}", l);
        }

        println!("========= >>>>>> col <<<<<< =========");
        let col_ruler = sudoku_loop[1];
        for l in col_ruler.partitions.iter() {
            println!("{:?}", l);
        }

        println!("========= >>>>>> matrix <<<<<< =========");
        let mut matrix = new_sudoku_matrix_value();
        let matrix_ruler = sudoku_loop[2];
        for (i, l) in matrix_ruler.partitions.iter().enumerate() {
            for (x, y) in l {
                matrix[*x][*y] = i;
            }
        }
        for l in matrix.iter() {
            println!("{:?}", l);
        }
    }
}
