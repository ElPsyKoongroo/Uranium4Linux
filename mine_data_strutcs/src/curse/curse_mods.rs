#![allow(non_snake_case)]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
/// This struct only contains data about the mod logo.
struct Logo {
    id: usize,
    modId: usize,
    thumbnailUrl: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// This struct contains the data about the specific file of a mod
pub struct CurseFile {
    id: usize,
    gameId: Option<usize>,
    modId: usize,
    displayName: String,
    fileName: PathBuf,
    downloadUrl: Option<String>,
    fileLength: usize,
    gameVersions: Vec<String>,
}

impl CurseFile {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_game_id(&self) -> usize {
        self.gameId.unwrap_or_default()
    }

    pub fn get_mod_id(&self) -> usize {
        self.modId
    }

    pub fn get_game_versions(&self) -> &Vec<String> {
        &self.gameVersions
    }

    pub fn get_display_name(&self) -> String {
        self.displayName.clone()
    }

    pub fn get_file_name(&self) -> PathBuf {
        self.fileName.clone()
    }

    pub fn get_download_url(&self) -> String {
        self.downloadUrl.clone().unwrap_or_default()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FingerPrintInfo {
    id: usize,
    pub file: CurseFile,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// This struct contains the data about the request of a fingerprint
/// Fingerprint requets are like
///:"data": {
///:    exactMatches: \[
///:        CurseFile
///:    \]
///:}
pub struct CurseFingerPrint {
    exactMatches: Vec<FingerPrintInfo>,
}

impl CurseFingerPrint {
    pub fn get_file(&self) -> &CurseFile {
        &self.exactMatches[0].file
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// This struct contains the data about a single version of a mod
pub struct CurseVersion {
    id: usize,
    gameId: usize,
    name: String,
    slug: String,
    downloadCount: usize,
    latestFiles: Vec<CurseFile>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// This struct contains the data about the multiple versions of a mod
pub struct CurseVersions {
    data: Vec<CurseVersion>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Because the standar response from Curse API is:
/// "data": {
///     * fields of other struct *
/// }
/// We need this struct.
pub struct CurseResponse<T: Serialize> {
    pub data: T,
}
