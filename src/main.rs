use sudoku::{
    algorithm::CandidateMatrix, entity::{
        is_sudoku_value, SudokuMatrixValue, SudokuValueType, SQUARE_INNER_LEN, SQUARE_OUTER_LEN,
        SUDOKU_UNKNOWN,
    }, guess::SudokuSolver, rulers::init
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
    for (i, line) in matrix.matrix.iter().enumerate() {
        println!(
            "{} {} {}  {} {} {}  {} {} {} ",
            line[0], line[1], line[2], line[3], line[4], line[5], line[6], line[7], line[8]
        );
        if i % SQUARE_INNER_LEN == SQUARE_INNER_LEN - 1 {
            println!();
        }
    }
    println!();
}

fn show_can(can: &CandidateMatrix) {
    for (i, line) in can.can_matrix.iter().enumerate() {
        for row in 0..SQUARE_INNER_LEN {
            for (j, c) in line.iter().enumerate() {
                for col in 0..SQUARE_INNER_LEN {
                    let value = row * SQUARE_INNER_LEN + col;
                    if c.can[value] {
                        print!("{} ", value + 1);
                    } else {
                        print!("  ");
                    }
                }
                if (j + 1) % SQUARE_INNER_LEN == 0 {
                    print!(" | ");
                } else {
                    print!("   ");
                }
            }
            println!();
        }
        if (i + 1) % SQUARE_INNER_LEN == 0 {
            for _ in 0..SQUARE_OUTER_LEN {
                print!("______   ");
            }
        } else {
            for _ in 0..SQUARE_OUTER_LEN {
                print!("       + ");
            }
        }
        println!();
    }
    println!();
}

/// > Get-Content .\input | .\sudoku.exe
fn main() -> std::io::Result<()> {
    let mut is_print_help = false;
    let mut is_debug_mode = false;
    let mut is_show_candi = false;
    for ele in std::env::args().skip(0) {
        match &ele as &str {
            "help" => is_print_help = true,
            "debug" => is_debug_mode = true,
            "candi" => is_show_candi = true,
            _ => {}
        }
    }
    is_show_candi &= is_debug_mode;

    if is_print_help {
        println!("input to stdin a file with sudoku");
        println!("option:");
        println!("help -> to print help");
        println!("debug -> to show SudokuMatrix each step");
        println!("candi -> to show CandidateMatrix each step, only if debug");
        return Ok(());
    }

    #[cfg(debug_assertions)]
    let input_data = {
        let mut path = std::env::current_dir()?;
        path.push("sudoku_matrix");
        std::fs::read_to_string(path)?
    };

    #[cfg(not(debug_assertions))]
    let input_data = {
        let mut input_data = String::new();
        std::io::stdin().read_to_string(&mut input_data)?;
        input_data
    };

    let sudoku = from_string(&input_data);
    println!("sudoku matrix is:");
    show(&sudoku);

    init();
    let mut can = CandidateMatrix::from(sudoku);
    loop {
        if can.finished() {
            println!("The only certain result is:");
            show(&can.into());
            return Ok(());
        }

        let origin = can.clone();
        can.evolution();
        can.evolution_by_position_mutex();
        can.evolution_by_check_position();
        if origin == can {
            break;
        }

        if is_debug_mode {
            show(&can.into());
        }
        if is_show_candi {
            show_can(&can);
        }
    }

    println!("All possible result is:");
    let mut soler = SudokuSolver::from(can);
    soler.solver_possible();

    for matrix in soler.get_all_possible_sudoku() {
        show(matrix);
    }
    Ok(())
}
