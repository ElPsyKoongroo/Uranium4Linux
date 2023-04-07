use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BASE: &'static str = "https://resources.download.minecraft.net/";

/*

            MINECRAFT ASSETS DATA STRUCTURES

*/

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ObjectData {
    pub hash: String,
    pub size: usize,
}

impl ObjectData {
    pub fn get_link(&self) -> String {
        format!("{}{}/{}", BASE, &self.hash[..2], self.hash)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadData {
    sha1: String,
    size: usize,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources {
    pub objects: HashMap<String, ObjectData>,
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Instancee<'a> {
    id: &'a str,
    downloads: HashMap<&'a str, DownloadData<'a>>,
}
*/

/*

       https://launchermeta.mojang.com/mc/game/version_manifest.json
                  MINECRAFT INSTANCES DATA STRUCTURE

*/
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MinecraftVersion {
    id: String,
    #[serde(rename = "type")]
    instance_type: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
}

impl MinecraftVersion {
    pub fn get_id_raw(&self) -> &str {
        &self.id
    }

    pub fn get_link_raw(&self) -> &str {
        &self.url
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MinecraftInstances {
    //latest: (String, String),
    pub versions: Vec<MinecraftVersion>,
}

impl MinecraftInstances {
    pub fn get_versions_raw(&self) -> &[MinecraftVersion] {
        &self.versions
    }
    pub fn get_instance_url(&self, instance: &str) -> Option<&str> {
        for version in &self.versions {
            if version.get_id_raw() == instance {
                return Some(version.get_link_raw());
            }
        }
        None
    }
}

/*


       MINECRAFT INSTANCE DATA STRUCTURE


*/

#[derive(Debug, Serialize, Deserialize, Default)]
struct Artifact {
    path: String,
    sha1: String,
    size: usize,
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LibData {
    downloads: Vec<Artifact>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Library {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssestIndex {
    pub id: String,
    pub sha1: String,
    pub size: usize,
    #[serde(rename = "totalSize")]
    pub total_size: u128,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinecraftInstance {
    #[serde(rename = "assetIndex")]
    pub assest_index: AssestIndex,
    pub id: String,
    pub downloads: HashMap<String, DownloadData>,
}
