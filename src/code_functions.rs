use std::io::Stderr;
use std::ops::Index;
use std::io::{Error, ErrorKind};

use tokio::fs::File;

use crate::{requester::request_maker::Requester};
use crate::easy_input::input;
use crate::minecraft::minecraft_mod::{RinthMod, RinthVersions};



pub enum CODES {
    ModSelected,
    PageSelected,
    ParseError
}

#[warn(dead_code)]
pub struct Properties{
    pub limit: u32,
    pub page: u32,
    pub offset: u32,
    pub selected_mod: usize
}


pub fn clear_screen(){
    print!("\x1B[2J\x1B[1;1H");
}

pub async fn download_mod(minecraft_mod_versions: &RinthVersions, requester: &Requester) -> Result<i32, Box<dyn std::error::Error>>{
   
    let index: i32 = input("Select the desiere version: ", -1);
   
    match index.is_negative(){
        false => {
            let response = requester.get(
                minecraft_mod_versions.get_version(index as usize).get_file_url()
            ).await?;
            let content = response.bytes().await?;
            let path = minecraft_mod_versions.get_version(index as usize).get_file_name();
            tokio::fs::write(path, content).await?;

            Ok(index)
        }

        true => {
            Err(Box::new(Error::new(ErrorKind::Other, "Bad input!")))
        }
    }
}