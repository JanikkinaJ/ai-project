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

    // Method to iterate over all placed queens
    fn iter_queens(&self) -> impl Iterator<Item = (usize, i8)> + '_ {
        self.board
            .iter() // Iterate over the board vector
            .enumerate() // Add column indices to the iterator
            .filter_map(|(col, row)| row.map(|r| (col, r))) // Filter `None` and map `Some(row)` to `(col, row)`
    }


    fn print_queens(&self) {
        let queens: Vec<String> = self
            .iter_queens() // Iterate through all placed queens
            .map(|(col, row)| format!("{{{}, {}}}", col, row)) // Format each as "{column, row}"
            .collect(); // Collect into a vector of strings

        println!("Queens: [{}]", queens.join(", "));
    }
    
    fn print_board_grid(&self) {
        println!("Current Board:");
        for row in 0..(self.size as i8) {
            for col in 0..self.size {
                if let Some(queen_row) = self.get(col) {
                    if queen_row == row {
                        print!(" Q "); // Queen position
                    } else {
                        print!(" . "); // Empty square
                    }
                } else {
                    print!(" . "); // Empty square
                }
            }
            println!(); // Newline after each row
        }
    }
    // Getter for a specific column
    fn get(&self, column: usize) -> Option<i8> {
        if column < self.size {
            self.board[column]
        } else {
            None
        }
    }

    // Setter to place a queen at a specific column and row
    fn set(&mut self, column: usize, row: i8) -> bool {
        if self.check_valid(column, row) {
            self.board[column] = Some(row);
            return true;
        } else {
            println!("Invalid position: column={}, row={}. Board size is {}.",column, row, self.size);
            return false;
        }
    }

    // uses all checks to check if coordinate is valid
    fn check_valid(&self, column :usize,row: i8) -> bool {
        if !(self.check_column(column as i8)) {
            println!("Column conflict: {column}:{row}");
                return false;
        } else if !(column < self.size && row >= 0 && row < self.size as i8) {
            println!("Size conflict: {column}:{row}");
                return false;
        } else if !(self.check_all_diagonal(column)) {
            println!("Diagonal conflict: {column}:{row}");
                return false;
        } 
        return true;
    }

    // Check if a column is safe (no queens on the same column) and report conflicts
    fn check_column(&self, column: i8) -> bool {
        if let Some((col, row)) = self
            .board
            .iter()
            .enumerate() // Include column indices
            .find(|&(_, &row)| row == Some(column))
        {
            println!("Conflict detected with column: {column}: Queen already in column {col} at row {row:?}");
            return false;
        }
        return true; // No conflicts found
    }

    // get diagonal condition
    // TODO change as currently it assumes both queens exist but this is used for checking potential queen
    fn check_diagonal(&self, queen1: usize, queen2: usize) -> bool {
        // check that both queens exist
        match (self.get(queen1), self.get(queen2)) {
            (Some(row1), Some(row2)) => {
                // Calculate differences
                let col_diff = (queen1 as i8 - queen2 as i8).abs();
                let row_diff = (row1 - row2).abs();
                println!("row and col diff: {row_diff} == {col_diff}");
                row_diff == col_diff // Return whether the 2 queens are diagonally aligned
            }
            _ => {
                println!("One or both queens do not exist at: queen1={queen1}, queen2={queen2}");
                return false;
            }
        }
    }

    fn check_all_diagonal(&self, queen1: usize) -> bool {
        for col in 0..(self.size-1) {
            if self.check_diagonal(queen1, col) {
                println!("{queen1} and {col} aligned");
                return false;
            }
        }
        println!("{queen1} not aligned diagonally");
        return true;
    }
}

fn main() -> Result<(), String> {
    let size = 8;
    let mut board = Board::new(size);

    let COLUMN_A: usize = 0;
    let ROW_A: i8 = 0;
    let COLUMN_B: usize = 1;
    let ROW_B: i8 = 1;

    // Place queens on the board
    board.set(COLUMN_A, ROW_A);
    board.set(COLUMN_B, ROW_B);

    board.print_board_grid();
    board.print_queens();
    
    Ok(())
}
