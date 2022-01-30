#![allow(dead_code)]
use core::fmt;
use std::fmt::format;

use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufWriter};

use super::minecraft_mod::*;

trait Jsoneable {
    fn to_json(&self) -> serde_json::Result<()>
    where Self: Serialize,
    {
        let file = File::create("mods.json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurseResponse {
    data: Vec<CurseMod>,
}
impl Jsoneable for CurseResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RinthResponse {
    pub hits: Vec<RinthMod>,
    offset: u32,
    limit: u32,
    total_hits: u64,
}
impl Jsoneable for RinthResponse {}

impl RinthResponse {
    pub fn new() -> RinthResponse {
        let rinth_res = RinthResponse {
            hits: vec![],
            offset: 0,
            limit: 0,
            total_hits: 0,
        };
        rinth_res
    }

    pub fn show(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for RinthResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, minecraft_mod) in self.hits.iter().enumerate() {
            write!(f, "{:2}: {}\n", index, minecraft_mod.to_string())?;
        }
        write!(f, "")
    }
}
