use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModpackError {
    #[error("Wrong file format")]
    WrongFileFormat,
    #[error("Wrong modpack format")]
    WrongModpackFormat,
    #[error("File not found")]
    FileNotFound,
}
