use crate::{dbg_only, settings::Difficulty, Board, DIM_SIZE};
use log::debug;
use rand::distributions::{Distribution, WeightedIndex};

pub fn gen_rnd_pre_formed(likelyness: Difficulty) -> Board {
    let mut board = [
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [6, 7, 2, 1, 9, 5, 3, 4, 8],
        [1, 9, 8, 3, 4, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    ];

    let accepted_values = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut weights = [1; 10];
    weights[0] = likelyness.value(); // let the number 0 be n times as common as everything else
                                     // rand weight dist config where n is the likelyness multiplier

    let dist = WeightedIndex::new(weights).unwrap();
    let mut rng = rand::thread_rng();

    board.iter_mut().for_each(|row: &mut [u8; DIM_SIZE]| {
        let mask: [bool; DIM_SIZE] =
            std::array::from_fn(|_| accepted_values[dist.sample(&mut rng)] > 0);

        dbg_only!({
            debug!("{mask:?}");
        });

        for (i, &toggle) in mask.iter().enumerate() {
            if !toggle {
                row[i] = 0;
            }
        }
    });

    board
}
