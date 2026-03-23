use std::fmt;
use clap::ValueEnum;
use serde::{Serialize, Deserialize};

#[derive(ValueEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Position {
    Start,
    Center,
    End,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Position::Start => write!(f, "Start"),
            Position::Center => write!(f, "Center"),
            Position::End => write!(f, "End"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_position() {
        assert_eq!(Position::Start.to_string(), "Start");
        assert_eq!(Position::Center.to_string(), "Center");
        assert_eq!(Position::End.to_string(), "End");
    }
}