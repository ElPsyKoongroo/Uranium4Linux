use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModpackError {
    #[error("Wrong file format")]
    WrongFileFormat,
    #[error("Wrong modpack format")]
    WrongModpackFormat,
    #[error("File not found")]
    FileNotFound,
    #[error("Cant create dir")]
    CantCreateDir,
}

#[derive(Debug, Error)]
pub enum MakerError {
    #[error("Cant compress the modpack")]
    CantCompress,
    #[error("Cant remove temp JSON file")]
    CantRemoveJSON

}
