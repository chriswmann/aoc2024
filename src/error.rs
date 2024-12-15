use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum AocError {
    FromStrError(String),
    ParseDirectionError(char),
    ParsePointTypeError(char),
}

impl std::error::Error for AocError {}
impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FromStrError(e) => writeln!(f, "Couldn't get rule from str, {}", e),
            Self::ParseDirectionError(e) => {
                writeln!(f, "Couldn't parse direction from char, {}", e)
            }
            Self::ParsePointTypeError(e) => {
                writeln!(f, "Couldn't parse point type from char, {}", e)
            }
        }
    }
}
