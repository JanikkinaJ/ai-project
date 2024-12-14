mod lib;

fn main() {
    let size = 8;
    match lib::solve_n_queens(size) {
        Some(solution) => {
            println!("Solution for {}-Queens:", size);
            lib::print_board(&solution);
        }
        None => println!("No solution found for {}-Queens.", size),
    }
}
