#[allow(dead_code)]
mod tests;
pub type Board = Vec<Vec<u8>>;

pub fn solve(board: &mut Board) -> bool {
    let empty_coords = find_empty(board);

    if empty_coords.len() == 0 {
        return true;
    }

    let (row, col) = empty_coords[0];
    for num in 1..10 {
        if is_valid(board, row, col, num) {
            board[row][col] = num;
            if solve(board) {
                return true;
            }
            board[row][col] = 0;
        }
    }
    return false;
}

pub fn find_empty(board: &Board) -> Vec<(usize, usize)> {
    let mut empty_corrds = Vec::<(usize, usize)>::new();
    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                empty_corrds.push((i, j));
            }
        }
    }
    return empty_corrds;
}

pub fn is_valid(board: &Board, row: usize, col: usize, num: u8) -> bool {
    for i in 0..9 {
        if board[row][i] == num {
            return false;
        }
    }
    for i in 0..9 {
        if board[i][col] == num {
            return false;
        }
    }
    let row_start = (row / 3) * 3;
    let col_start = (col / 3) * 3;
    for i in row_start..row_start + 3 {
        for j in col_start..col_start + 3 {
            if board[i][j] == num {
                return false;
            }
        }
    }
    return true;
}

pub fn print_board(board: &Board) {
    for row in 0..board.len() {
        if row % 3 == 0 {
            println!("- - - - - - - - - - - - - -");
        }
        for col in 0..board[0].len() {
            if col % 3 == 0 {
                print!(" | ");
            }

            if col == 8 {
                print!("{}\n", board[row][col]);
            } else {
                print!("{} ", board[row][col]);
            }
        }
    }
}
