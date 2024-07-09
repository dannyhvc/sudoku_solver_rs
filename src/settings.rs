#[derive(Debug, Clone, Copy, Default)]
pub enum Difficulty {
    Easy,
    #[default]
    Medium,
    Hard,
}

impl Difficulty {
    pub fn value(self) -> i32 {
        match self {
            Difficulty::Easy => 3,
            Difficulty::Medium => 6,
            Difficulty::Hard => 9,
        }
    }
}
