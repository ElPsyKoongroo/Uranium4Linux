#![allow(dead_code)]

use std::{fs::File, io::BufWriter};
use serde::{Serialize, Deserialize};
use crate::minecraft_mod;

trait Jsoneable{ 
    fn to_json(&self) -> serde_json::Result<()> 
        where Self: Serialize{
            let file = File::create("mods.json").unwrap();
            let mut writer = BufWriter::new(file);
            serde_json::to_writer_pretty(&mut writer, &self)?;
            Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurseResponse{
    data: Vec<minecraft_mod::CurseMod>,
}
impl Jsoneable for CurseResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RinthResponse{
    hits: Vec<minecraft_mod::RinthMod>,
    offset: u32,
    limit: u32,
    total_hits: u64
}
impl Jsoneable for RinthResponse {}