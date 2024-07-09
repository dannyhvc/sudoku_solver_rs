use std::collections::HashMap;
use std::hash::Hash;

use log::{debug, info};
use rand::distributions::{Distribution, WeightedIndex};

use crate::iter::ColumnIterator;
use crate::settings::Difficulty;
use crate::validator::SudokuValidator;
use crate::{dbg_only, Board, Coords, DIM_SIZE, SUBG};

/// Trait for generating and solving Sudoku boards.
pub trait SudokuSolver: SudokuValidator {
    /// Solves the Sudoku puzzle using a backtracking algorithm.
    ///
    /// # Returns
    ///
    /// Returns `true` if the puzzle is solvable and solved, otherwise `false`.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use sudoku_solver::{Board, SudokuGenerator};
    ///
    /// let mut board = Board::default();
    /// board[0][0] = 5;
    /// assert!(board.solve());
    /// ```
    fn solve(&mut self) -> bool;

    /// Finds all empty cells (cells with a value of 0) in the Sudoku board.
    ///
    /// # Returns
    ///
    /// A `Vec` containing the coordinates of all empty cells.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use sudoku_solver::{Board, Coords, SudokuGenerator};
    ///
    /// let board = Board::default();
    /// let empty_cells = board.find_empty();
    /// assert!(!empty_cells.is_empty());
    /// ```
    fn find_empty(&self) -> Vec<Coords>;

    /// Prints the Sudoku board to the console.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use sudoku_solver::{Board, SudokuGenerator};
    ///
    /// let board = Board::default();
    /// board.print_board();
    /// ```
    fn print_board(&self);
}

pub trait SudokuGenerator {
    fn generate_unsolved_sudoku(difficulty: Difficulty) -> Board;
    fn remove_column_duplicates(&mut self, coords: Coords);
    fn remove_row_duplicates(&mut self, coords: Coords);
    fn remove_subgrid_duplicates(&mut self, coords: Coords);
    fn remove_all_duplicates(&mut self);
}

impl SudokuSolver for Board {
    /// Finds all the empty cells (cells with a value of 0) in a 2D array.
    ///
    /// # Returns
    ///
    /// A Vec containing the coordinates of all empty cells. Each coordinate
    /// is represented as a tuple `(i, j)`, where `i` is the row index and `j`
    /// is the column index.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// let grid = [
    ///     [1, 0, 3],
    ///     [4, 5, 0],
    ///     [7, 8, 9]
    /// ];
    /// let empty_cells = grid.find_empty();
    /// assert_eq!(empty_cells, vec![(0, 1), (1, 2)]);
    /// ```
    ///
    /// # Implementation Details
    ///
    /// This function iterates over each cell in the 2D array. If a cell's value
    /// is 0, it records the cell's coordinates in a vector.
    ///
    /// # Note
    ///
    /// This function assumes that `self` implements `IntoIterator` and its
    /// `IntoIter` produces items that also implement `IntoIterator`.
    fn find_empty(&self) -> Vec<Coords> {
        // Create a vector to store the coordinates of empty cells
        let mut empty_coords = Vec::<Coords>::new();

        // Iterate over each row and its index
        self.iter().enumerate().for_each(|(row_idx, row)| {
            // Iterate over each cell in the row and its index
            row.iter().enumerate().for_each(|(col_idx, &cell)| {
                // If the cell is empty (value equals 0), add its coordinates to the vector
                if cell == 0 {
                    empty_coords.push((row_idx, col_idx));
                }
            });
        });

        // Return the vector containing the coordinates of empty cells
        empty_coords
    }

    /// Attempts to solve the Sudoku puzzle using backtracking.
    ///
    /// # Returns
    ///
    /// `true` if the puzzle is solved successfully, `false` otherwise.
    ///
    /// The function works as follows:
    /// 1. It finds the coordinates of all empty cells on the board.
    /// 2. If there are no empty cells, it means the board is solved, so it returns `true`.
    /// 3. If there are empty cells, it attempts to fill the first empty cell with a valid number (1..=9).
    /// 4. For each number, it checks if placing the number in the cell is valid.
    /// 5. If valid, it places the number in the cell and recursively attempts to solve the rest of the board.
    /// 6. If the recursive call solves the board, it returns `true`.
    /// 7. If not, it resets the cell to empty (0) and tries the next number.
    /// 8. If no number is valid for the cell, it returns `false`, triggering backtracking.
    fn solve(&mut self) -> bool {
        // Find all empty cell coordinates on the board
        let empty_coords = self.find_empty();

        // Short-circuit: if no empty cells, the board is solved
        if empty_coords.is_empty() {
            dbg_only!({
                println!("Solved");
            });
            return true;
        }

        // Get the first empty cell coordinates
        let (row, col) = empty_coords[0];

        // logging HACK!
        dbg_only!({
            debug!(
                "current Empty coords: {:?} \tworking on coord {:?}",
                empty_coords.len(),
                (row, col)
            );
        });

        // Try placing each number from 1 to DIM_SIZE in the empty cell
        for value_for_sudoku in 1..=DIM_SIZE as u8 {
            // Check if placing the number is valid
            let validity = self.is_valid((row, col), value_for_sudoku);
            if validity.is_ok() {
                // Place the number in the cell
                self[row][col] = value_for_sudoku;

                // Recursively attempt to solve the rest of the board
                if self.solve() {
                    return true;
                }

                // If not solved, reset the cell and backtrack
                self[row][col] = 0;
            }
        }

        // If no valid number can solve the board, return false
        false
    }

    fn print_board(&self) {
        const END_INDEX: usize = DIM_SIZE - 1;
        for (row_index, row) in self.iter().enumerate().take(DIM_SIZE) {
            if row_index % SUBG == 0 {
                println!("- - - - - - - - - - - - - -");
            }
            for (col_index, &cell) in row.iter().enumerate().take(DIM_SIZE) {
                if col_index % SUBG == 0 {
                    print!(" | ");
                }

                match col_index == END_INDEX {
                    true => println!("{}", cell),
                    false => print!("{} ", cell),
                }
            }
        }
    }
}

impl SudokuGenerator for Board {
    /// Generates an unsolved Sudoku board based on the given difficulty level.
    ///
    /// This function first generates a solvable Sudoku board, then removes random positions
    /// to create a puzzle with the specified difficulty.
    ///
    /// # Parameters
    /// - `difficulty`: The difficulty level of the puzzle to be generated.
    ///
    /// # Returns
    /// A `Board` representing the unsolved Sudoku puzzle.
    fn generate_unsolved_sudoku(difficulty: Difficulty) -> Board {
        // We need to make sure the board is solvble first then we'll "remove" pseudo
        // random positions from the board to make it make it a puzzle
        let mut board: Board = crate::rand::gen_rnd_pre_formed(difficulty);

        // creating a rand weighted gaus-dist using accepted values and weights for
        // the creation of the samples
        const ACCEPTED: [u8; 2] = [0, 1];
        let weights = [1, difficulty.value()];

        // rand config
        let mut rng = rand::thread_rng();
        let dist = WeightedIndex::new(weights).unwrap();

        (0..DIM_SIZE).for_each(|row_idx| {
            let row_postion_mask: [u8; DIM_SIZE] =
                std::array::from_fn(|_| ACCEPTED[dist.sample(&mut rng)]);

            row_postion_mask.iter().enumerate().for_each(
                |(col_idx, &switch)| {
                    if !switch == 0 {
                        board[row_idx][col_idx] = 0u8;
                    }
                },
            );
        });
        // board.remove_all_duplicates();
        board
    }

    /// Removes duplicates in a specified column of the Sudoku board.
    ///
    /// This function identifies duplicate values in the specified column and removes
    /// them by setting their values to 0, leaving only the first occurrence of each value.
    ///
    /// # Parameters
    /// - `coords`: A tuple representing the coordinates (row, column) used to specify the column.
    ///
    /// # Panics
    /// This function will panic if the coordinates are out of bounds.
    fn remove_column_duplicates(&mut self, coords: Coords) {
        let col_iter = ColumnIterator::new(self, coords.1);
        // max mem size is DIM_SIZE * size_of(usize)
        let mut duplicate_counter: HashMap<u8, Vec<usize>> =
            find_duplicates(col_iter.cloned());

        // the function will all so accumulate on duplicates with value 0 but in
        // our case 0 means empty so the key 0 is obsolete
        duplicate_counter.remove(&0);

        let changes: Vec<usize> = duplicate_counter
            .iter_mut()
            .filter(|(_k, v)| v.len() > 1)
            .flat_map(|(_k, v)| v.iter().skip(1).cloned())
            .collect();

        for &x in &changes {
            self[x][coords.1] = 0;
        }
    }

    /// Removes duplicates in a specified row of the Sudoku board.
    ///
    /// This function identifies duplicate values in the specified row and removes
    /// them by setting their values to 0, leaving only the first occurrence of each value.
    ///
    /// # Parameters
    /// - `coords`: A tuple representing the coordinates (row, column) used to specify the row.
    ///
    /// # Panics
    /// This function will panic if the coordinates are out of bounds.
    fn remove_row_duplicates(&mut self, coords: Coords) {
        // solution from chatgpt
        let row_iter = self[coords.0].iter().cloned();
        let mut duplicate_counter: HashMap<u8, Vec<usize>> =
            find_duplicates(row_iter);

        // the function will all so accumulate on duplicates with value 0 but in
        // our case 0 means empty so the key 0 is obsolete
        duplicate_counter.remove(&0);

        let changes: Vec<usize> = duplicate_counter
            .iter_mut()
            .filter(|(_k, v)| v.len() > 1)
            .flat_map(|(_k, v)| v.iter().skip(1).cloned())
            .collect();

        for &x in &changes {
            self[coords.0][x] = 0;
        }
    }

    /// Removes duplicates in the subgrid containing the specified coordinates.
    ///
    /// This function identifies duplicate values in the subgrid and removes them by
    /// setting their values to 0, leaving only the first occurrence of each value.
    ///
    /// # Parameters
    /// - `coords`: A tuple representing the coordinates (row, column) used to specify the subgrid.
    ///
    /// # Panics
    /// This function will panic if the coordinates are out of bounds.
    fn remove_subgrid_duplicates(&mut self, coords: Coords) {
        let subgrid_start_row = (coords.0 / SUBG) * SUBG;
        let subgrid_start_col = (coords.1 / SUBG) * SUBG;

        let mut duplicate_counter: HashMap<u8, Vec<(usize, usize)>> =
            HashMap::new();

        // Iterate over the subgrid and collect duplicates
        for (r_index, row) in
            self.iter().enumerate().skip(subgrid_start_row).take(SUBG)
        {
            for (c_index, &elem) in
                row.iter().enumerate().skip(subgrid_start_col).take(SUBG)
            {
                if elem != 0 {
                    duplicate_counter.entry(elem).or_default().push((
                        r_index + subgrid_start_row,
                        c_index + subgrid_start_col,
                    ));
                }
            }
        }

        // Remove duplicates, only keep the first occurrence
        duplicate_counter
            .values_mut()
            .filter(|v| v.len() > 1)
            .flat_map(|v| v.iter().skip(1))
            .for_each(|&(r, c)| self[r][c] = 0);
    }

    fn remove_all_duplicates(&mut self) {
        // named constants to help rationalize the grid
        const TOP: usize = 0;
        const MID: usize = 1;
        const BOTTOM: usize = 2;
        const LEFT: usize = TOP;
        const CENTER: usize = MID;
        const RIGHT: usize = BOTTOM;

        // remove subgrid duplicates first
        [
            (TOP * SUBG, LEFT * SUBG),
            (TOP * SUBG, CENTER * SUBG),
            (TOP * SUBG, RIGHT * SUBG),
            (MID * SUBG, LEFT * SUBG),
            (MID * SUBG, CENTER * SUBG),
            (MID * SUBG, RIGHT * SUBG),
            (BOTTOM * SUBG, LEFT * SUBG),
            (BOTTOM * SUBG, CENTER * SUBG),
            (BOTTOM * SUBG, RIGHT * SUBG),
        ]
        .iter()
        .for_each(|c| {
            self.remove_subgrid_duplicates(*c);

            // FOR DEBUG
            dbg_only!({
                debug!(
                    "\n{}",
                    crate::validator::gen_subgrid_debug_string(self, *c)
                );
                debug!("{}", self.is_subgrid_valid(*c, 3));
            });
        });

        // remove row and column duplicates
        for i in 0..DIM_SIZE {
            self.remove_row_duplicates((i, 0));
            self.remove_column_duplicates((0, i));
        }
    }
}

pub fn find_duplicates<I, T>(iter: I) -> HashMap<T, Vec<usize>>
where
    I: Iterator<Item = T>,
    T: Eq + Hash + Copy,
{
    let mut duplicate_counter: HashMap<T, Vec<usize>> = HashMap::new();

    for (i, elem) in iter.enumerate() {
        duplicate_counter.entry(elem).or_default().push(i);
    }

    duplicate_counter
}
