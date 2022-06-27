#[derive(Clone, Debug)]
pub enum FileType {
    Data,
    Dir,
    Other,
}

#[derive(Clone)]
pub struct UraniumFile{
    /// Relative path from minecraft root mods/sodium.jar
    path: String, 
    /// sodium.jar  
    name: String,
    file_type: FileType,
}


impl UraniumFile {
    pub fn new(path: &str, name: &str, file_type: FileType) -> UraniumFile {
        UraniumFile {
            path: path.to_owned(),
            name: name.to_owned(),
            file_type,
        }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }
    
    pub fn get_name(&self) -> String {
        self.name.clone()
    } 

    pub fn get_absolute_path(&self) -> String {
        self.path.clone() + &self.name
    }

    //setter for type_file
    pub fn set_type(&mut self, _file_type: FileType) {
        self.file_type = _file_type;
    }

    pub fn get_type(&self) -> FileType {
        self.file_type.clone()
    }

}