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
            .filter_map(|(row, col)| col.map(|c| (row, c))) // Filter `None` and map `Some(col)` to `(col, row)`
    }


    fn get_queens(&self) -> String {
        let queens: Vec<String> = self
            .iter_queens() // Iterate through all placed queens
            .map(|(row, col)| format!("{{{}, {}}}", row, col)) // Format each as "{column, row}"
            .collect(); // Collect into a vector of strings
        
        format!("Queens: [{}]", queens.join(", "))
    }

    fn print_queens(&self) {
        println!("{}", self.get_queens());
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
    fn set(&mut self, row: usize, column: i8) -> bool {
        if self.check_valid(row, column) {
            self.board[row] = Some(column);
            println!("Creating Queen at ({row},{column})");
            return true;
        } else {
            println!("Invalid position: column={}, row={}. Board size is {}.",column, row, self.size);
            return false;
        }
    }

    // uses all checks to check if coordinate is valid
    fn check_valid(&self, row :usize,column: i8) -> bool {
        if !(self.check_column(column)) {
            println!("Column conflict: {row}:{column}");
                return false;
        } else if !(row < self.size && column >= 0 && column < self.size as i8) {
            println!("Size conflict: {row}:{column}");
                return false;
        } else if !self.check_all_diagonal(row, column) {
            println!("Diagonal conflict: {row}:{column}");
                return false;
        } 
        return true;
    }

    fn check_column(&self, column: i8) -> bool {
    for (row_id, &col) in self.board.iter().enumerate() {
        if col == Some(column) {
            println!(
                "Conflict detected with column: {column}: \
                 Queen already in column {column} at row {row:?}",
                row = row_id
            );
            return false;
        }
    }
    true // No conflicts found
}

    // returns true if diagonal is fine
    fn check_diagonal(&self, row_one: usize, col_one: i8, row_two: usize, col_two: i8) -> bool {
        // Calculate differences
        let col_diff = (col_one - col_two).abs();
        let row_diff = (row_one as i8 - row_two as i8).abs();
        //println!("row and col diff: {row_diff} == {col_diff}");
        !(row_diff == col_diff) // Return whether the diagonal placement is valid
    }

    // returns true if all queen diagonals don't conflict with coord
    fn check_all_diagonal(&self, row: usize, col: i8) -> bool {
        for q_row in 0..(self.size-1) {
            match self.get(q_row) {
                Some(q_col) => { 
                    if !self.check_diagonal(row, col, q_row, q_col) {
                        println!("{q_row} and {row} aligned");
                        return false;
                    }
                },
                _ => print!("")

            }
        }
        //println!("queens not aligned diagonally");
        return true;
    }
}

fn main() -> Result<(), String> {
    let size = 8;
    let mut board = Board::new(size);

    let row_a: usize = 0;
    let col_a: i8 = 0;
    let row_b: usize = 2;
    let col_b: i8 = 3;

    // Place queens on the board
    board.set(row_a, col_a);
    board.set(row_b, col_b);

    board.print_board_grid();
    board.print_queens();
    
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
