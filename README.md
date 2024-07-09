# Sudoku Solver (Rust)

## Overview

This is a text-based Sudoku solver program implemented in Rust. It uses the backtracking algorithm to find the solution to any given Sudoku board.

## Features

Solves Sudoku puzzles of varying difficulty levels.
Simple and intuitive command-line interface.

## Getting Started
### Prerequisites

- [Rust](https://www.rust-lang.org/)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/dannyhvc/sudoku_solver_rs.git
```

2. Navigate to the project directory:

```bash
cd sudoku_solver_rs
```

3. Build the project:

```bash
cargo build --release
```

## Usage
1. Run the program:

```bash
./target/release/sudoku_solver_rs
```

2. Enter the Sudoku puzzle:

* Input the Sudoku board row by row, using '0' to represent empty cells.

* Example input:
```rust
    let input: Board = vec![
        vec![0, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![4, 0, 6, 7, 8, 9, 1, 2, 3],
        vec![7, 8, 0, 1, 2, 3, 4, 5, 6],
        vec![2, 3, 4, 0, 6, 7, 8, 9, 1],
        vec![5, 6, 7, 8, 0, 1, 2, 3, 4],
        vec![8, 9, 1, 2, 3, 0, 5, 6, 7],
        vec![3, 4, 5, 6, 7, 8, 0, 1, 2],
        vec![6, 7, 8, 9, 1, 2, 3, 0, 5],
        vec![9, 1, 2, 3, 4, 5, 6, 7, 0],
    ];
```
3. View the solution:
* The program will output the solved Sudoku board.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
Inspired by the joy of solving Sudoku puzzles.
Thanks to the Rust programming language and its community.

### Error Hypothesis
Heres what I think is happening and why I think its not solving the current
sudoku. It errors because there is a dup, but then doesn't change it because
the dup is clearly `0` every time and the system doesn't seem to check for
that branching case.

### Solution Hypothesis:
Probably look into solving the error case and perhaps better error logging
when in debug mode. Also, it might be worth investing in using color_eyre
so that context can be added when an error is thrown.

### Trial logs:
1) so i tried filtering the 0s from the validation checking but it seems
like it only ended up solving for the subgrid and and for the row or col

2) Good news, found out through some better investigation that even if there
are sudoku boards with less than 17 clues than the board has multiple solutions
in terms of either it has multiple solutions or it has none. Although giving
more hints seems like the right thing to do, if not thought out carefully it
can give 0 solutions. Often anything more than 20 hints gives only 1 solution, but
anything less than 17 gives unknown number of solutions within 1..=9**81
