mod test_fixtures;

use crate::{
    generator::{SudokuGenerator, SudokuSolver},
    settings::Difficulty,
    Board,
};
use rstest::rstest;
use test_fixtures::*;

// // #[rstest]
// fn test_find_empty(preset_diagonal_zeros: Vec<Coords>) {
//     let input: Board = [
//         [0, 2, 3, 4, 5, 6, 7, 8, 9],
//         [4, 0, 6, 7, 8, 9, 1, 2, 3],
//         [7, 8, 0, 1, 2, 3, 4, 5, 6],
//         [2, 3, 4, 0, 6, 7, 8, 9, 1],
//         [5, 6, 7, 8, 0, 1, 2, 3, 4],
//         [8, 9, 1, 2, 3, 0, 5, 6, 7],
//         [3, 4, 5, 6, 7, 8, 0, 1, 2],
//         [6, 7, 8, 9, 1, 2, 3, 0, 5],
//         [9, 1, 2, 3, 4, 5, 6, 7, 0],
//     ];

//     for i in 0..preset_diagonal_zeros.len() {
//         assert_eq!(
//             preset_diagonal_zeros[i].0 as u8,
//             input.find_empty()[i].0 as u8,
//             "Expected {}",
//             preset_diagonal_zeros[i].0
//         );
//     }
// }

// // #[rstest]
// fn test_is_valid(error_sudoku: Board) {
//     let should_be_true = error_sudoku.is_valid((0, 3), 2);
//     let should_be_false = error_sudoku.is_valid((0, 3), 1);
//     assert!(should_be_true.is_ok());
//     assert!(should_be_false.is_err());
// }

// // #[rstest]
// fn test_solve(preset_expected_solution_0: (Board, Board)) {
//     let (mut actual, expected) = preset_expected_solution_0;

//     // actual.print_board();
//     assert!(actual.solve());
//     for i in 0..DIM_SIZE {
//         for j in 0..DIM_SIZE {
//             assert_eq!(
//                 actual[i][j], expected[i][j],
//                 "Expected {}",
//                 expected[i][j]
//             );
//         }
//     }
//     // actual.print_board();
//     // expected.print_board();
// }

// // #[rstest]
// fn test_solve_from_single_number(preset_single_number: (Board, Board)) {
//     let mut actual: Board = preset_single_number.0;
//     let expected: Board = preset_single_number.1;

//     assert!(actual.solve());
//     for i in 0..DIM_SIZE {
//         for j in 0..DIM_SIZE {
//             // println!(
//             //     "actual: {:?} -- expected: {:?}",
//             //     actual[i][j], expected[i][j]
//             // );
//             assert_eq!(
//                 actual[i][j], expected[i][j],
//                 "Expected {}",
//                 expected[i][j]
//             );
//         }
//     }

//     // actual.print_board();
// }

// // #[rstest]
// fn test_solve_preset_valid_ones(mut preset_valid_ones: Board) {
//     assert!(preset_valid_ones.solve());
//     // preset_valid_ones.print_board();
// }

// // #[rstest]
// fn test_gen(_logger: ()) {
//     // env_logger::init();
//     let mut board: Board = Board::generate_unsolved_sudoku(Difficulty::Easy);
//     board.solve();

//     // println!("\niteration 1");
//     // board.print_board()
// }

/// see [README](README.md)

#[rstest]
fn test_remove_column_duplicates(_logger: (), mangled_board: Board) {
    let mut board: Board = Board::generate_unsolved_sudoku(Difficulty::Hard);
    // let mut board = mangled_board;
    // board.print_board();
    println!();

    // removing all dups isn't currnetly working
    board.remove_all_duplicates();
    println!("Unsolved board");
    board.print_board();

    println!();
    board.solve();
    board.print_board();
}
