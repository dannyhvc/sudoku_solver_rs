/// An iterator for iterating over a specific column in a 2D matrix.
///
/// This iterator allows you to traverse elements in a specific column
/// of a square matrix (a matrix with equal number of rows and columns).
///
/// # Type Parameters
///
/// - `'a`: The lifetime of the reference to the matrix.
/// - `T`: The type of the elements in the matrix.
/// - `ROWS`: The number of rows (and columns) in the matrix.
///
/// # Fields
///
/// - `matrix`: A reference to the 2D matrix.
/// - `col_index`: The index of the column to iterate over.
/// - `row_index`: The current row index being iterated.
///
/// # Example
///
/// ```rust ignore
/// let matrix = [
///     [1, 2, 3],
///     [4, 5, 6],
///     [7, 8, 9]
/// ];
/// let col_iter = ColumnIterator::new(&matrix, 1);
/// let column: Vec<_> = col_iter.collect();
/// assert_eq!(column, vec![&2, &5, &8]);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ColumnIterator<'a, T, const ROWS: usize> {
    pub(crate) matrix: &'a [[T; ROWS]; ROWS],
    pub(crate) col_index: usize,
    pub(crate) row_index: usize,
}

impl<'a, T, const ROWS: usize> ColumnIterator<'a, T, ROWS> {
    /// Creates a new `ColumnIterator` for the specified column index.
    ///
    /// # Arguments
    ///
    /// - `matrix`: A reference to the 2D matrix.
    /// - `col_index`: The index of the column to iterate over.
    ///
    /// # Returns
    ///
    /// A new instance of `ColumnIterator`.
    pub(crate) fn new(matrix: &'a [[T; ROWS]; ROWS], col_index: usize) -> Self {
        Self {
            matrix,
            col_index,
            row_index: 0,
        }
    }
}

impl<'a, T, const ROWS: usize> Iterator for ColumnIterator<'a, T, ROWS> {
    type Item = &'a T;

    /// Advances the iterator and returns the next element in the column.
    ///
    /// # Returns
    ///
    /// - `Some(&T)`: The next element in the column if it exists.
    /// - `None`: If the iterator has reached the end of the column.
    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index < self.matrix.len() {
            let item = &self.matrix[self.row_index][self.col_index];
            self.row_index += 1;
            Some(item)
        } else {
            None
        }
    }
}
