#![allow(useless_deprecated)]
// use crate::board::settings::Difficulty;
pub type Board = [[u8; DIM_SIZE]; DIM_SIZE];
pub type Coords = (usize, usize);

/// Total square grid dimension
pub const DIM_SIZE: usize = 9;

/// Sub Grid Dimension
pub const SUBG: usize = 3;

pub mod generator;
pub mod iter;
pub mod rand;
pub mod settings;
pub mod validator;

#[cfg(test)]
mod test;

#[macro_export]
macro_rules! dbg_only {
    ($e:expr) => {
        if cfg!(debug_assertions) {
            $e
        }
    };
}
