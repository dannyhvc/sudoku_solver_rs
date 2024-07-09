# Sudoku Solver RS

## Overview

The `sudoku_solver_rs` is a comprehensive library for Sudoku puzzles. It originated from the initial version of a Sudoku solving program and has evolved to offer functionalities to solve and generate Sudoku puzzles. The library will soon be able to extract difficulty data from a Sudoku, opening up opportunities for its implementation across various applications.

## Features

- **Sudoku Solving**: Solve any valid Sudoku puzzle.
- **Sudoku Generation**: Generate new Sudoku puzzles.
- **Difficulty Extraction** (Coming Soon): Extract difficulty data from a Sudoku puzzle.

## Installation

Extract the repository to your chosen directory.
Add this to your `Cargo.toml`:

```toml
[dependencies]
sudoku_solver_rs = { path="../path/to/sudoku_solver_rs" }
```

Then import it in your project:

```rust
use sudoku_solver_rs;
```

## Usage

Here's a simple example of how to use the library:

```rust
// Sample Program:
use sudoku_solver_rs::{
    generator::{SudokuGenerator, SudokuSolver},
    settings::Difficulty,
    Board,
};

fn main() {
    let mut board: Board = Board::generate_unsolved_sudoku(Default::default());

    println!("Unsolved board\n");
    board.print_board();

    // if solved it will display the solved board
    if board.solve() {
        println!("Solved board\n");
        board.print_board();
    }
}
```

## Contributing

Contributions are welcome! Please read our [contributing guidelines](LINK_TO_GUIDELINES) for details.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

