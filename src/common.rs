use std::fmt::Display;

pub struct Guess {
    pub id: u32,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct IncorrectStringLenError {
    size: usize,
}

impl IncorrectStringLenError {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

impl Display for IncorrectStringLenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "incorrect size [size={}]", self.size)
    }
}
