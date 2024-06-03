use std::io::Read;

use sudoku::{
    algorithm::CandidateMatrix,
    entity::{is_sudoku_value, SudokuMatrixValue, SudokuValueType, SQUARE_OUTER_LEN, SUDOKU_UNKNOWN},
    rulers::init,
};

mod sudoku;

fn from_string(s: &String) -> SudokuMatrixValue {
    assert!(s.is_ascii());
    let mut matrix = SudokuMatrixValue::new();
    let (mut row, mut col) = (0, 0);
    for value_origin in s.chars().map(|c| c.to_digit(10)) {
        let value: SudokuValueType = value_origin.unwrap_or(u32::MAX) as SudokuValueType;
        if value == SUDOKU_UNKNOWN || is_sudoku_value(value) {
            if col >= SQUARE_OUTER_LEN {
                col = 0;
                row += 1;
            }
            if row >= SQUARE_OUTER_LEN {
                break;
            }
            matrix.matrix[row][col] = value;
            col += 1;
        }
    }
    matrix
}

fn show(matrix: &SudokuMatrixValue) {
    for line in matrix.matrix.iter() {
        println!(
            "{} {} {} {} {} {} {} {} {} ",
            line[0], line[1], line[2], line[3], line[4], line[5], line[6], line[7], line[8]
        );
    }
}

/// > Get-Content .\input | .\sudoku.exe
fn main() -> std::io::Result<()> {
    let mut input_data = String::new();
    std::io::stdin().read_to_string(&mut input_data)?;

    let sudoku = from_string(&input_data);
    show(&sudoku);

    init();
    let mut can: CandidateMatrix = sudoku.into();
    loop {
        let mut finished = true;
        for ll in can.can_matrix.iter() {
            for can in ll.iter() {
                finished &= can.only().is_some();
            }
        }
        if finished {
            break;
        }

        println!("\n\n");
        can.evolution();
        can.evolution_by_position_mutex();
        can.evolution_by_check_position();
        show(&Into::<SudokuMatrixValue>::into(can));
    }

    Ok(())
}
