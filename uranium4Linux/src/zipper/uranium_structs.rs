use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub enum FileType {
    Data,
    Dir,
    Other,
}

#[derive(Clone)]
pub struct UraniumFile {
    /// Relative path from minecraft root mods/sodium.jar
    path: PathBuf,
    /// sodium.jar  
    name: String,
    file_type: FileType,
}

impl UraniumFile {
    pub fn new<I: AsRef<Path>>(path: I, name: &str, file_type: FileType) -> UraniumFile {
        UraniumFile {
            path: path.as_ref().to_path_buf(),
            name: name.to_owned(),
            file_type,
        }
    }

    pub fn get_path(&self) -> String {
        self.path.as_os_str().to_str().unwrap_or_default().to_string()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_absolute_path(&self) -> String {
        self.path.join(&self.name).to_str().unwrap_or_default().to_string()
    }

    pub fn set_type(&mut self, new_file_type: FileType) {
        self.file_type = new_file_type;
    }

    pub fn get_type(&self) -> FileType {
        self.file_type.clone()
    }
}
