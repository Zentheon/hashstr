use std::fmt::Display;

#[derive(Debug)]
pub struct LenError {
    pub expected: usize,
    pub got: usize,
    pub hash_name: String,
}

#[derive(Debug)]
pub struct EncodingError {
    pub hash_name: String,
}

#[derive(Debug)]
pub enum Error {
    LenError(LenError),
    EncodingError(EncodingError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LenError(e) => write!(f, "{e}"),
            Self::EncodingError(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for LenError {}
impl std::error::Error for EncodingError {}
impl std::error::Error for Error {}

impl Display for LenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid length of {}. Expected: {}, got: {}",
            self.hash_name, self.expected, self.got
        )
    }
}

impl Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Characters in {} should be hexadecimal", self.hash_name)
    }
}
