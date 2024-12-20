#[derive(Debug)]
struct Board {
    size: usize,
    board: Vec<Option<i8>>, // Each row points to a column (None if no queen is present)
}

impl Board {
    /// Constructor to initialize the board
    fn new(size: usize) -> Self {
        Self {
            size,
            board: vec![None; size],
        }
    }

    /// Getter for column of queen in provided row
    fn get(&self, row: usize) -> Option<i8> {
        if row < self.size  {
            self.board[row]
        } else {
            None
        }
    }

    /// checks for column conflicts
    fn check_column(&self, column: i8) -> bool {
        !self.board.iter().any(|&col| col == Some(column))
    }

    /// returns true if diagonal is fine
    fn check_diagonal(&self, row_one: usize, col_one: i8, row_two: usize, col_two: i8) -> bool {
        // Calculate differences
        let col_diff = (col_one - col_two).abs();
        let row_diff = (row_one as i8 - row_two as i8).abs();
        //println!("row and col diff: {row_diff} == {col_diff}");
        !(row_diff == col_diff) // Return whether the diagonal placement is valid
    }

    /// returns true if all queen diagonals don't conflict with coord
    fn check_all_diagonal(&self, row: usize, col: i8) -> bool {
        for q_row in 0..(self.size-1) {
            if let Some(q_col) = self.get(q_row) {//handles the case where q_col is None
                if !self.check_diagonal(row, col, q_row, q_col) {
                    return false;
                }
            }
        }
        return true;
    }

    /// uses all checks to check if coordinate is valid
    fn check_valid(&self, row :usize,column: i8) -> bool {
        if !(self.check_column(column)) {
            return false;
        } else if !(row < self.size && column >= 0 && column < self.size as i8) {
            return false;
        } else if !self.check_all_diagonal(row, column) {
            return false;
        }
        return true;
    }

    /// Setter to place a queen at a specific column and row
    fn set(&mut self, row: usize, column: i8) -> bool {
        if self.check_valid(row, column) {
            self.board[row] = Some(column);
            return true;
        } else {
            println!("Invalid Queen position: column={}, row={}. Board size is {}.",column, row, self.size);
            return false;
        }
    }

    /// unsetter to remove a queen at a specific column and row
    fn unset(&mut self, row: usize, column: i8) -> bool {
        if !(row < self.size && column >= 0 && column < self.size as i8) {
            println!("Invalid position: row={}, column={}. Board size is {}.",row, column, self.size);
            return false;
        } else {
            self.board[row] = None;
            return true
        }
    }

    /// Get state of all queens via a string
    fn get_queens(&self) -> String {
        let queens: Vec<String> = self
            .board
            .iter()
            .enumerate() // Get (row, Option<column>) for each row
        // Map to formatted string if column exists
            .filter_map(|(row, &col)| col.map(|c| format!("{{{}, {}}}", row, c)))
            .collect();
        format!("Queens: [{}]", queens.join(", "))
    }

    /// print queens state to terminal
    fn print_queens(&self) {
        println!("{}", self.get_queens());
    }

    /// nicer printing of current queen state
    fn print_board_grid(&self) {
        print!("===");
        for i in 0..self.size {
            print!("{i}==")
        }
        println!();
        for col in 0..self.size {
            print!("{col}|");
            for row in 0..(self.size as i8) {
                if let Some(queen_col) = self.get(col) {
                    if queen_col == row {
                        print!(" Q "); // Queen
                    } else {
                        print!(" . "); // Empty
                    }
                } else {
                    print!(" . "); // Empty
                }
            }
            println!("|"); // Newline after each row
        }
        for _i in 0..self.size {
            print!("===")
        }
        println!("===");
    }

}

/// the backtrack solution for queen 8x8 problem
fn backtrack(board: &mut Board, row: usize, solutions: &mut Vec<String>) -> i32 {
    if row == board.size {
        solutions.push(board.get_queens().to_string());
        board.print_board_grid();
        board.print_queens();
        return 1
    }
    let mut count = 0;
    for col in 0..board.size as i8  {
        if board.check_valid(row, col) {
            board.set(row, col);
            count = count + backtrack(board, row + 1, solutions);
            board.unset(row, col);
        }
    }
    return count
}

/// solve 8x8 queens
fn solve_n_queens(n: usize) -> i32 {
    let mut board = Board::new(n);
    let mut solutions: Vec<String> = Vec::new();
    if n <= 10 {
        let solution_count = backtrack(&mut board, 0, &mut solutions);
        println!("Solved {n}x{n} Queens problem with {solution_count} solutions!");
        return solution_count;
    } else {
        println!("Solving for larger than 10 ahsn't been implemented yet");
        let solution_count = 0;
        return solution_count;
    }
}

fn main() -> Result<(), String> {
    solve_n_queens(8);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_queen() {
        let mut board =  Board::new(8);
        let create = board.set(0, 0);
        assert_eq!(create, true);
    }

    #[test]
    fn diagonal_found() {
        let mut board =  Board::new(8);
        board.set(0, 0);
        let bad_coord = board.set(1, 1);
        assert_eq!(bad_coord, false);
        assert_eq!(board.check_diagonal(0,0, 1,1), false);
    }

    #[test]
    fn conflicting_column_found() {
        let mut board =  Board::new(8);
        board.set(1, 2);
        assert_eq!(board.check_column(1),true);
        assert_eq!(board.check_column(2),false);
        assert_eq!(board.check_column(3),true);
    }

    #[test]
    fn too_big() {
        let board =  Board::new(8);
        let too_big = board.check_valid(0, 8);
        let other_too_big = board.check_valid(8, 0);
        let way_too_big = board.check_valid(8, 8);
        let okay_size = board.check_valid(0, 0);
        assert_eq!(too_big, false);
        assert_eq!(other_too_big, false);
        assert_eq!(way_too_big, false);
        assert_eq!(okay_size,true);
    }

    #[test]
    fn too_small() {
        let board =  Board::new(8);
        let too_small = board.check_valid(0, -1);
        let way_too_small = board.check_valid(0, -90);
        assert_eq!(too_small, false);
        assert_eq!(way_too_small, false);
    }

    #[test]
    fn output_test() {
        let mut board =  Board::new(8);
        board.set(0, 0);
        let mut queens = board.get_queens();
        assert_eq!(queens,"Queens: [{0, 0}]");
        board.set(1, 4);
        queens = board.get_queens();
        assert_eq!(queens,"Queens: [{0, 0}, {1, 4}]");
        board.set(2, 7);
        board.set(3, 5);
        queens = board.get_queens();
        assert_eq!(queens,"Queens: [{0, 0}, {1, 4}, {2, 7}, {3, 5}]");
        board.set(4, 2);
        board.set(5, 6);
        queens = board.get_queens();
        assert_eq!(queens,"Queens: [{0, 0}, {1, 4}, {2, 7}, {3, 5}, {4, 2}, {5, 6}]");
        board.set(6, 1);
        board.set(7, 3);
        queens = board.get_queens();
        assert_eq!(queens,"Queens: [{0, 0}, {1, 4}, {2, 7}, {3, 5}, {4, 2}, {5, 6}, {6, 1}, {7, 3}]");
        println!("Test getter:");
        assert_eq!(board.get(0), Some(0));
        assert_eq!(board.get(1), Some(4));
        assert_eq!(board.get(2), Some(7));
        assert_eq!(board.get(3), Some(5));
        assert_eq!(board.get(4), Some(2));
        assert_eq!(board.get(5), Some(6));
        assert_eq!(board.get(6), Some(1));
        assert_eq!(board.get(7), Some(3));
    }

}
