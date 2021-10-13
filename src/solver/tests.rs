# [allow(dead_code, unused_imports)]
pub(crate) use crate::solver::*;

#[test]
fn test_find_empty() {
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
    let create_expected = || { 0..9 }.map(|i| (i, i)).collect();
    let expected: Vec<(u8, u8)> = create_expected();

    for i in 0..expected.len() {
        assert_eq!(
            expected[i].0 as u8,
            find_empty(&input)[i].0 as u8,
            "Expected {} to be {}",
            find_empty(&input)[i].0,
            expected[i].0
        );
    }
}

#[test]
fn test_is_valid() {
    let error_sudoku: Board = vec![
        vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0],
    ];
    let should_be_true = is_valid(&error_sudoku, 0, 3, 2);
    let should_be_false = is_valid(&error_sudoku, 0, 3, 1);
    assert_eq!(true, should_be_true);
    assert_eq!(false, should_be_false);
}

#[test]
fn test_solve() {
    let mut actual: Board = vec![
        vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
        vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
        vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
        vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
        vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
        vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
        vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
        vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    let expected: Board = vec![
        vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
        vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
        vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
        vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
        vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
        vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
        vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
        vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
        vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
    ];

    assert!(solve(&mut actual));
    for i in 0..9 {
        for j in 0..9 {
            assert_eq!(
                actual[i][j], expected[i][j],
                "Expected {} to be {}",
                actual[i][j], expected[i][j]
            );
        }
    }
}
