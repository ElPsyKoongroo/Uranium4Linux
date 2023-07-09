#![allow(non_snake_case)]

use core::fmt;

use serde::{Deserialize, Serialize};

pub enum Attributes {
    Loader,
    Name,
    VersionType,
}

// This structures pretends be the same format as
// Rinth API json responses so in order to easily mantein
// the code all the attributes should be private so if
// somewhen the name of an attributes change only the
// attribute inside the structure will change but the setter/getter
// will be the same.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchProjects {
    hits: Vec<RinthProject>,
}

/// `RinthMod` pretends to be the structure for the response of
/// https://api.modrinth.com/v2/project/{id | slug}
/// This type is also usable when requesting searchs for rinth api
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RinthProject {
    project_id: Option<String>,
    title: String,
    description: String,
    downloads: u32,
    versions: Vec<String>,
    categories: Vec<String>,
    icon_url: String,
}

impl std::fmt::Display for RinthProject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Mod name: {}", self.title)
    }
}

impl RinthProject {
    pub fn get_id(&self) -> String {
        self.project_id.clone().unwrap()
    }

    pub fn get_versions(&self) -> &Vec<String> {
        &self.versions
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Dependency {
    version_id: Option<String>,
    project_id: Option<String>,
    dependency_type: String,
}

impl Dependency {
    pub fn get_project_id(&self) -> &str {
        match self.project_id {
            Some(ref id) => id,
            None => "",
        }
    }

    pub fn get_version_id(&self) -> &str {
        match self.version_id {
            Some(ref id) => id,
            None => "",
        }
    }
}

/// `RinthProject` pretends to be the response for:
/// https://api.modrinth.com/v2/version/{version id}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RinthVersion {
    id: String,
    project_id: String,
    name: String,
    version_type: String,
    downloads: u64,
    files: Vec<RinthFile>,
    dependencies: Vec<Dependency>,
    game_versions: Vec<String>,
    loaders: Vec<String>,
}

impl RinthVersion {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_versions(&self) -> &Vec<String> {
        &self.game_versions
    }

    pub fn get_versions_usize(&self) -> Vec<usize> {
        self.game_versions
            .iter()
            .flat_map(|v| v.replace('.', "").parse::<usize>())
            .collect()
    }

    pub fn get_file_url(&self) -> String {
        self.files[0].url.clone()
    }

    pub fn get_file_name(&self) -> String {
        self.files[0].filename.clone()
    }

    pub fn get_hashes(&self) -> &Hashes {
        &self.files[0].hashes
    }

    pub fn get_size(&self) -> usize {
        self.files[0].size
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_loader(&self) -> String {
        self.loaders[0].clone()
    }

    pub fn get_project_id(&self) -> String {
        self.project_id.clone()
    }

    pub fn is_fabric(&self) -> bool {
        self.get_loader() == "fabric"
    }

    pub fn had_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }

    pub fn get_dependencies(&self) -> &Vec<Dependency> {
        &self.dependencies
    }
}

/// Rinthversions pretends to parse the response of:
/// https://api.modrinth.com/v2/project/{id | slug}/version
/// This structure is commonly use
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RinthVersions {
    pub versions: Vec<RinthVersion>,
}

impl std::convert::From<Vec<RinthVersion>> for RinthVersions {
    fn from(versions: Vec<RinthVersion>) -> RinthVersions {
        RinthVersions { versions }
    }
}

impl RinthVersions {
    pub fn new() -> RinthVersions {
        RinthVersions {
            versions: Vec::new(),
        }
    }

    pub fn get_version(&self, i: usize) -> &RinthVersion {
        &self.versions[i]
    }

    pub fn len(&self) -> usize {
        self.versions.len()
    }

    pub fn push(&mut self, version: RinthVersion) {
        self.versions.push(version);
    }

    pub fn last(&self) -> Option<&RinthVersion> {
        self.versions.last()
    }

    pub fn first(&self) -> Option<&RinthVersion> {
        self.versions.first()
    }

    pub fn mod_at(&self, i: usize) -> &RinthVersion {
        &self.versions[i]
    }

    pub fn versions(&self) -> &[RinthVersion] {
        &self.versions
    }

    pub fn filter_by(&self, attribute: &Attributes, content: &str) -> Vec<RinthVersion> {
        match attribute {
            Attributes::Loader => self
                .versions
                .iter()
                .filter(|x| x.loaders.iter().any(|c| c == content))
                .cloned()
                .collect::<Vec<RinthVersion>>(),

            Attributes::Name => self
                .versions
                .iter()
                .filter(|x| x.name.contains(&content.to_string()))
                .cloned()
                .collect::<Vec<RinthVersion>>(),

            Attributes::VersionType => self
                .versions
                .iter()
                .filter(|x| x.version_type.contains(&content.to_string()))
                .cloned()
                .collect::<Vec<RinthVersion>>(),
        }
    }

    pub fn has(&self, id: &str) -> bool {
        self.versions.iter().any(|x| x.project_id == id)
    }
}

impl fmt::Display for RinthVersions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut max_width = {
            self.versions
                .iter()
                .map(|x| x.name.clone().len())
                .max()
                .unwrap_or(0)
        };
        let vt_len = "version type".chars().count();

        if max_width < vt_len {
            max_width = vt_len;
        }

        write!(
            f,
            "\n\nindex\t{:<max_width$}\tversion type\tdownloads\n",
            "version name"
        )?;

        for (index, version) in self.versions.iter().enumerate() {
            writeln!(
                f,
                "{index:^5}\t{:<max_width$}\t{:^12}\t{:>9}",
                version.name, version.version_type, version.downloads
            )?;
        }
        write!(f, "")
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Hashes {
    pub sha512: String,
    pub sha1: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct RinthFile {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub size: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RinthResponse {
    pub hits: Vec<RinthProject>,
    offset: u32,
    limit: u32,
    total_hits: u64,
}

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
        println!("{self}");
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
            writeln!(f, "{:2}: {}", index, minecraft_mod)?;
        }
        write!(f, "")
    }
}

/*
/// If a is newer -1, if b is newer 1, if they are the same 0
pub fn compare_versions(a: &RinthProject, b: &RinthProject) -> i8 {
    let a_version = a.get_versions()[0].clone();
    let b_version = b.get_versions()[0].clone();
    eprintln!("{} - {}", a_version, b_version);
    for (a_number, b_number) in a_version.split('.').zip(b_version.split('.')) {
        if a_number == b_number {
            continue;
        }

        let a_number = a_number.parse::<u8>().unwrap();
        let b_number = b_number.parse::<u8>().unwrap();

        if a_number > b_number {
            return -1;
        }
        return 1;
    }
    0
}
*/
