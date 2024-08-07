#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("move file error: {0}")]
    MoveFileError(String),
    #[error("keyword length error: {0}")]
    KeywordLengthError(String),
    #[error("io error: {0}")]
    IOError(String),
    #[error("parse error: {0}")]
    ParseError(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.to_string())
    }
}
