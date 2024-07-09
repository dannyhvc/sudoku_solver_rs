use crate::{dbg_only, iter, Board, Coords, SUBG};
use log::*;

/// Trait for validating Sudoku boards.
pub trait SudokuValidator {
    /// Checks if placing a value at the given coordinates is valid.
    ///
    /// # Arguments
    ///
    /// * `coords` - The coordinates to check.
    /// * `value` - The value to place.
    ///
    /// # Returns
    ///
    /// Returns `true` if placing the value is valid, otherwise `false`.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use crate::{Board, Coords, SudokuValidator};
    ///
    /// let board = Board::default();
    /// assert!(board.is_row_valid((0, 0), 1));
    /// ```
    fn is_row_valid(&self, coords: Coords, value: u8) -> bool;

    /// Checks if placing a value in the given column is valid.
    ///
    /// # Arguments
    ///
    /// * `coords` - The coordinates to check.
    /// * `value` - The value to place.
    ///
    /// # Returns
    ///
    /// Returns `true` if placing the value is valid, otherwise `false`.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use crate::{Board, Coords, SudokuValidator};
    ///
    /// let board = Board::default();
    /// assert!(board.is_col_valid((0, 0), 1));
    /// ```
    fn is_col_valid(&self, coords: Coords, value: u8) -> bool;

    /// Checks if placing a value in the subgrid is valid.
    ///
    /// # Arguments
    ///
    /// * `coords` - The coordinates to check.
    /// * `value` - The value to place.
    ///
    /// # Returns
    ///
    /// Returns `true` if placing the value is valid, otherwise `false`.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use crate::{Board, Coords, SudokuValidator};
    ///
    /// let board = Board::default();
    /// assert!(board.is_subgrid_valid((0, 0), 1));
    /// ```
    fn is_subgrid_valid(&self, coords: Coords, value: u8) -> bool;

    /// Checks if placing a value at the given coordinates is valid.
    ///
    /// # Arguments
    ///
    /// * `coords` - The coordinates to check.
    /// * `value` - The value to place.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if placing the value is valid, or `Err(reason)` if invalid.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use crate::{Board, Coords, SudokuValidator};
    ///
    /// let board = Board::default();
    /// assert!(board.is_valid((0, 0), 1).is_ok());
    /// ```
    fn is_valid(&self, coords: Coords, value: u8) -> Result<(), &str> {
        dbg_only!({
            debug!("\n===== Attempting <value>: {value} =====");
        });
        let valid_row = self.is_row_valid(coords, value);
        let valid_col = self.is_col_valid(coords, value);
        let valid_sub = self.is_subgrid_valid(coords, value);

        let validity = match (valid_row, valid_col, valid_sub) {
            (true, true, true) => Ok(()),
            (true, true, false) => Err("Invalid subgrid"),
            (true, false, true) => Err("Invalid column"),
            (true, false, false) => Err("Invalid column and subgrid"),
            (false, true, true) => Err("Invalid row"),
            (false, true, false) => Err("Invalid row and subgrid"),
            (false, false, true) => Err("Invalid row and column"),
            (false, false, false) => Err("Invalid row, column, and subgrid"),
        };
        dbg_only!({
            if validity.is_ok() {
                info!("{validity:?}\n");
            } else {
                warn!("{validity:?}\n");
            }
        });
        validity
    }
}

// implementation for the default sudoku board
impl SudokuValidator for Board {
    fn is_row_valid(&self, coords: Coords, value: u8) -> bool {
        // making sure all the elements are different in the row
        dbg_only!({
            debug!(
                "\nchecking row: ({})\nvalues in row: {:?}",
                coords.0, self[coords.0]
            )
        });
        self[coords.0]
            .iter()
            // .filter(|&y| *y == 0)
            .all(|&y| y != value)
    }

    fn is_col_valid(&self, coords: Coords, value: u8) -> bool {
        let mut iter = iter::ColumnIterator::new(self, coords.1);
        // making sure all the elements are different in the column
        dbg_only!({
            debug!(
                "\nchecking column: ({})\nvalues in col: {:?}",
                coords.1,
                iter.collect::<Vec<_>>()
            );
        });

        iter
            // .filter(|&x| *x == 0)
            .all(|&x| x != value)
    }

    fn is_subgrid_valid(&self, (row, col): Coords, value: u8) -> bool {
        // Integer division by SUBG gives us the subgrid location
        // muld SUBG gives us the start location given the subgrid
        let row_start = (row / SUBG) * SUBG;
        let col_start = (col / SUBG) * SUBG;

        dbg_only!({
            debug!(
                "\nchecking subgrid: ({},{})\n{}",
                (row / SUBG),
                (col / SUBG),
                gen_subgrid_debug_string(self, (row_start, col_start))
            );
        });

        for row in self.iter().skip(row_start).take(SUBG) {
            for &subgrid_elem in row.iter().skip(col_start).take(SUBG) {
                if subgrid_elem == value {
                    return false;
                }
            }
        }
        true
    }
}

pub(crate) fn gen_subgrid_debug_string(
    board: &Board,
    (start_row, start_col): Coords,
) -> String {
    let mut debug_string = String::new();
    board.iter().skip(start_row).take(SUBG).for_each(|row| {
        row.iter()
            .skip(start_col)
            .take(SUBG)
            .for_each(|&subgrid_elem| {
                debug_string.push_str(&format!(" {}", subgrid_elem));
            });
        debug_string.push('\n');
    });

    debug_string
}
