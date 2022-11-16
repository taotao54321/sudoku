use sudoku::*;

fn main() -> anyhow::Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let board: Board = input.parse()?;

    let mut sudoku = Sudoku::new(board);

    if !sudoku.is_unique_solvable() {
        println!("WARN: not uniquely solvable");
    }

    if sudoku.solve() {
        print!("{}", sudoku.board());
    } else {
        println!("NO SOLUTION");
    }

    Ok(())
}
