use std::collections::HashMap;

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

pub const RULER_COUNT: usize = 3;
/// 三大规则：行、列、九宫格内数字不重复
pub type RulerLoop = [SudokuRuler; RULER_COUNT];
fn gen_ruler_loop() -> RulerLoop {
    let mut ruler_loop: RulerLoop = [SudokuRuler {
        partitions: [[(0, 0); SQUARE_OUTER_LEN]; SQUARE_OUTER_LEN],
    }; RULER_COUNT];

    // row
    for row in 0..SQUARE_OUTER_LEN {
        for col in 0..SQUARE_OUTER_LEN {
            ruler_loop[0].partitions[row][col] = (row, col);
        }
    }

    // column
    for col in 0..SQUARE_OUTER_LEN {
        for row in 0..SQUARE_OUTER_LEN {
            ruler_loop[1].partitions[col][row] = (row, col);
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
                    ruler_loop[2].partitions[each_num][element_num] = (row, col);
                    element_num += 1;
                }
            }

            each_num += 1;
        }
    }

    ruler_loop
}

pub type RulerPartitionMap = HashMap<Position, [[Position; SQUARE_OUTER_LEN]; RULER_COUNT]>;
fn gen_ruler_partition_map(ruler_loop: &RulerLoop) -> RulerPartitionMap {
    let mut map: RulerPartitionMap = HashMap::new();
    for row in 0..SQUARE_OUTER_LEN {
        for col in 0..SQUARE_OUTER_LEN {
            map.insert((row, col), [[(0, 0); SQUARE_OUTER_LEN]; RULER_COUNT]);
        }
    }

    for (i, ruler) in ruler_loop.iter().enumerate() {
        for partition in ruler.partitions.iter() {
            for position in partition.iter() {
                map.get_mut(position).unwrap()[i] = *partition;
            }
        }
    }

    map
}

struct RulerContainer {
    ruler_loop: RulerLoop,
    partition_map: RulerPartitionMap,
}

static mut RULER_CONTAINER: Option<RulerContainer> = None;
pub fn init() {
    let ruler_loop = gen_ruler_loop();
    unsafe {
        RULER_CONTAINER = Some(RulerContainer {
            ruler_loop,
            partition_map: gen_ruler_partition_map(&ruler_loop),
        })
    }
}

pub fn get_sudoku_ruler_loop() -> RulerLoop {
    unsafe { RULER_CONTAINER.as_ref().unwrap().ruler_loop }
}

pub fn each_sudoku_partition<F>(mut cb: F)
where
    F: FnMut(usize, &PositionPartition),
{
    let ruler_loop = get_sudoku_ruler_loop();
    for (ruler_id, ruler) in ruler_loop.iter().enumerate() {
        for partition in ruler.partitions.iter() {
            cb(ruler_id, partition);
        }
    }
}

pub fn get_sudoku_ruler_partition_map(
    pos: &Position,
) -> [[Position; SQUARE_OUTER_LEN]; RULER_COUNT] {
    unsafe {
        *RULER_CONTAINER
            .as_ref()
            .unwrap()
            .partition_map
            .get(pos)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::entity::SudokuMatrixValue;

    use super::*;

    #[test]
    fn test() {
        let sudoku_loop = gen_ruler_loop();

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
        let mut matrix_value = SudokuMatrixValue::new();
        let matrix_ruler = sudoku_loop[2];
        for (i, l) in matrix_ruler.partitions.iter().enumerate() {
            for (x, y) in l {
                matrix_value.matrix[*x][*y] = i;
            }
        }
        for l in matrix_value.matrix.iter() {
            println!("{:?}", l);
        }
    }
}
