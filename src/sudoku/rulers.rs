use super::entity::{MATRIX_INNER_COUNT, MATRIX_INNER_LEN, MATRIX_LEN};

/// 元素的位置
pub type SudokuPosition = (usize, usize);
/// 划分后同一系列元素
pub type SudokuCollection = [SudokuPosition; MATRIX_LEN];
/// 根据规则进行不同划分 行以行分 列以列分
pub type SudokuRuler = [SudokuCollection; MATRIX_LEN];
/// 三大规则：行、列、九宫格内数字不重复
pub type SudokuLoop = [SudokuRuler; 3];

pub const INIT_POS: SudokuPosition = (0, 0);
pub const INIT_COL: SudokuCollection = [INIT_POS; MATRIX_LEN];
pub const INIT_RUL: SudokuRuler = [INIT_COL; MATRIX_LEN];
pub const INIT_LOP: SudokuLoop = [INIT_RUL; 3];

pub static mut SUDOKU_LOOP: Option<SudokuLoop> = None;
pub fn init_sudoku_loop() {
    unsafe {
        SUDOKU_LOOP = Some(gen_sudoku_loop());
    }
}
pub fn gen_sudoku_loop() -> SudokuLoop {
    let mut sudoku_loop = INIT_LOP;

    // row
    for row in 0..MATRIX_LEN {
        for col in 0..MATRIX_LEN {
            sudoku_loop[0][row][col] = (row, col);
        }
    }

    // column
    for col in 0..MATRIX_LEN {
        for row in 0..MATRIX_LEN {
            sudoku_loop[1][col][row] = (row, col);
        }
    }

    // matrix
    let mut each_num = 0;
    for row_m in 0..MATRIX_INNER_COUNT {
        for col_m in 0..MATRIX_INNER_COUNT {
            let row_start = row_m * MATRIX_INNER_LEN;
            let row_final = row_m * MATRIX_INNER_LEN + MATRIX_INNER_LEN;
            let col_start = col_m * MATRIX_INNER_LEN;
            let col_final = col_m * MATRIX_INNER_LEN + MATRIX_INNER_LEN;

            let mut element_num = 0;
            for row in row_start..row_final {
                for col in col_start..col_final {
                    sudoku_loop[2][each_num][element_num] = (row, col);
                    element_num += 1;
                }
            }

            each_num += 1;
        }
    }

    sudoku_loop
}

#[cfg(test)]
mod tests {
    use crate::sudoku::entity::new_nine_nine_matrix;

    use super::*;

    #[test]
    fn test() {
        let sudoku_loop = gen_sudoku_loop();
        
        println!("========= >>>>>> row <<<<<< =========");
        let row_ruler = sudoku_loop[0];
        for l in row_ruler.iter() {
            println!("{:?}", l);
        }
        
        println!("========= >>>>>> col <<<<<< =========");
        let col_ruler = sudoku_loop[1];
        for l in col_ruler.iter() {
            println!("{:?}", l);
        }
        
        println!("========= >>>>>> matrix <<<<<< =========");
        let mut matrix = new_nine_nine_matrix();
        let matrix_ruler = sudoku_loop[2];
        for (i, l) in matrix_ruler.iter().enumerate() {
            for (x, y) in l {
                matrix[*x][*y] = i;
            }
        }
        for l in matrix.iter() {
            println!("{:?}", l);
        }
    }
}
