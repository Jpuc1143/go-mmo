use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidMoveError;

impl Display for InvalidMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid move")
    }
}

impl Error for InvalidMoveError {}
