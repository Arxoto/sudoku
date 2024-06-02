use sudoku::{
    algorithm::CandidateMatrix,
    entity::{is_sudoku_value, SudokuMatrixValue, SudokuValueType},
    rulers::init,
};

mod sudoku;

fn from_string(s: &String) -> SudokuMatrixValue {
    assert!(s.is_ascii());
    let mut matrix = SudokuMatrixValue::new();
    for (row, line) in s.split_whitespace().enumerate() {
        assert_eq!(line.len(), 9);
        for (col, value_origin) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            let value: SudokuValueType = value_origin.try_into().unwrap();
            if is_sudoku_value(value) {
                matrix.matrix[row][col] = value;
            }
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

fn main() -> std::io::Result<()> {
    const RADIX: u32 = 10;
    let x = "134";
    println!(
        "{}",
        x.chars().map(|c| c.to_digit(RADIX).unwrap()).sum::<u32>()
    );

    let mut path = std::env::current_dir()?;
    println!("The current directory is {}", path.display());
    path.push("input");
    if !path.exists() {
        println!("Input to {}", path.display());
        std::fs::File::create(path)?;
        return Ok(());
    }
    if !path.is_file() {
        println!("Not file {}", path.display());
        return Ok(());
    }
    let input_data = std::fs::read_to_string(path)?;

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
