use sudoku::*;

fn main() {
    let (mut sudoku, solution) = Sudoku::generate_unique(0);
    assert!(sudoku.is_unique_solvable());

    println!("{}", sudoku.board());
    println!("{}", solution.board());
}
