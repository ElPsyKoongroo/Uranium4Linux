use serde::{Serialize, Deserialize};
use std::fmt;

trait Downloadeable{
    fn download(){}
}




#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CurseMod{
    id: u64,
    name: String,
    downloadCount: f64
}

impl fmt::Debug for CurseMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&self.name)
         .field("id: ", &self.id)
         .field("downloads: ", &self.downloadCount)
         .finish()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RinthMod{
    mod_id: String,
    title: String,
    versions: Vec<String>,
    downloads: u32
}
