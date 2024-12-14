const ROW_A: i8 = 4;
const COLUMN_A: usize = 0;
const ROW_B: i8 = 6;
const COLUMN_B: usize = 2;


fn initialize_board(size: usize) -> Vec<Option<i8>> {
    let mut board: Vec<Option<i8>> = vec![None::<i8>; size]; // The board, None indicates no queens placed in the row.

    // assign coords
    board[COLUMN_A] = Some(ROW_A); // (0,4)
    board[COLUMN_B] = Some(ROW_B); // (2,6)
    board
}

fn check_column(board: &Vec<Option<i8>>, column :i8) -> bool {
  !board.contains(&Some(column))
}

// get diagonal condition
fn check_diagonal(board: &Vec<Option<i8>>, queen1: usize, queen2: usize) -> Result<bool, String> {
    // check that both queens exist
    match (board[queen1], board[queen2]) {
        (Some(row1), Some(row2)) => {
            // q1 row - q2 row                            q1 col - q2 col
            let col_diff = ((queen1 - queen2) as i8).abs();
            let row_diff = ((row1 - row2) as i8).abs();
            return Ok(row_diff == col_diff);
        }
        _ => {
            return Err("One of the queens used for comparison doesn't exist: queen1:{queen1}, queen2 {queen2}".to_string());
        }
    }
}

fn main() -> Result<(), String> {
    let size = 8;
    let board = initialize_board(size);
    match check_diagonal(&board, COLUMN_A, COLUMN_B)? {
        true => println!(
            "~Result: {COLUMN_A:?}:{:?} matches {COLUMN_B:?}:{:?}, it does.~",
            &board[COLUMN_A], &board[COLUMN_B]
        ),
        false => println!(
            "~Result: {COLUMN_A:?}:{:?} matches {COLUMN_B:?}:{:?}, it doesn't.~",
            &board[COLUMN_A], &board[COLUMN_B]
        ),
    }
    return Ok(());
}
