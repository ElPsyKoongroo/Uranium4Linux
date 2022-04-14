use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

use super::minecraft_mod::*;

trait Jsoneable {
    fn to_json(&self) -> serde_json::Result<()>
    where
        Self: Serialize,
    {
        let file = "mods.json";
        let mut output = File::create(file).unwrap();
        let s_json = serde_json::to_string_pretty(&self)?;
        write!(output, "{}", s_json).unwrap();
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
        RinthResponse {
            hits: vec![],
            offset: 0,
            limit: 0,
            total_hits: 0,
        }
    }

    pub fn show(&self) {
        println!("{}", self);
    }

    pub fn len(&self) -> usize {
        self.hits.len()
    }
}

impl std::default::Default for RinthResponse {
    fn default() -> RinthResponse {
        RinthResponse::new()
    }
}

impl std::fmt::Display for RinthResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (index, minecraft_mod) in self.hits.iter().enumerate() {
            write!(f, "{:2}: {}\n", index, minecraft_mod.to_string())?;
        }
        write!(f, "")
    }
}
