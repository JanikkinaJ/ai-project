const ROW_A: i8 = 4;
const COLUMN_A: usize = 0;
const ROW_B: i8 = 6;
const COLUMN_B: usize = 2;

#[derive(Debug)]
struct Board {
    size: usize,
    board: Vec<Option<i8>>, // Each column points to a row (None if no queen is present)
}

impl Board {
    // Constructor to initialize the board
    fn new(size: usize) -> Self {
        Self {
            size,
            board: vec![None; size],
        }
    }

    // Getter for a specific column
    fn get(&self, column: usize) -> Result<i8, String> {
        if column < self.size {
            match self.board[column] {
                Some(row) => Ok(row), // If a queen exists, return the row
                None => Err(format!("No queen at column {}", column)), // If no queen is found, return an error
            }
        } else {
            Err(format!(
                "Invalid position: column={} exceeds board size {}.",
                column, self.size
            ))
        }
    }

    // Setter to place a queen at a specific column and row
    fn set(&mut self, column: usize, row: i8) -> Result<(), String> {
        if self.check_valid(column, row) {
            self.board[column] = Some(row);
            return Ok(())
        } else {
            return Err(format!("Invalid position: column={}, row={}. Board size is {}.",column, row, self.size))
        }
    }

    // uses all checks to check if coordinate is valid
    fn check_valid(&self, column :usize,row: i8) -> bool{
        let check1 = self.check_column(column as i8);
        let check2 = column <= self.size && row > 0 && row <= self.size as i8;
        // also check diagonal
        check1 && check2
    }
    
    // Check if a column is safe (no queens on the same row)
    fn check_column(&self, column :i8) -> bool {
        !self.board.contains(&Some(column))
    }

    // get diagonal condition
    fn check_diagonal(&self, queen1: usize, queen2: usize) -> Result<bool, String> {
        // check that both queens exist
        let row1 = self.get(queen1)?;
        let row2 = self.get(queen2)?;
        // Calculate differences
        let col_diff = (queen1 as i8 - queen2 as i8).abs();
        let row_diff = (row1 - row2).abs();

        Ok(row_diff == col_diff) // Return whether the 2 queens are diagonally aligned 
    }
}

fn main() -> Result<(), String> {
    let size = 8;
    let mut board = Board::new(size);

    // Place queens on the board
    board.set(COLUMN_A, ROW_A)?;
    board.set(COLUMN_B, ROW_B)?;

    // Check if queens are on the same diagonal
    match board.check_diagonal(COLUMN_A, COLUMN_B)? {
        true => println!(
            "~Result: {COLUMN_A:?}:{:?} matches {COLUMN_B:?}:{:?}, it does.~",
            board.get(COLUMN_A),
            board.get(COLUMN_B)
        ),
        false => println!(
            "~Result: {COLUMN_A:?}:{:?} matches {COLUMN_B:?}:{:?}, it doesn't.~",
            board.get(COLUMN_A),
            board.get(COLUMN_B)
        ),
    }

    Ok(())
}
