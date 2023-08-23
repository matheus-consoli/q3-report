use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing argument: no log file provided")]
    NoInputFile,
    #[error("`{1}` does not exist")]
    FileNotFound(#[source] io::Error, String),
    // TODO: improve error information, provide the failing context
    #[error("error while parsing the log file")]
    Parsing(),
}
