use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CurseMod {
    id: u64,
    name: String,
    downloadCount: f64,
}

pub enum Attributes {
    Loader,
    Name,
    VersionType,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RinthMod {
    project_id: Option<String>,
    title: String,
    game_versions: Vec<String>,
    description: String,
    downloads: u32,
    versions: Vec<String>,
    categories: Vec<String>,
}

impl RinthMod {
    pub fn get_id(&self) -> String {
        //let id = self.project_id.clone().unwrap().split_off(6);
        self.project_id.clone().unwrap()
    }

    pub fn to_string(&self) -> String {
        format!("Mod name: {}", self.title)
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dependency{
    version_id: Option<String>,
    project_id: Option<String>,
    dependency_type: String
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
    
#[derive(Clone, Debug, Serialize, Deserialize)]
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

    pub fn get_file_url(&self) -> String {
        self.files[0].url.clone()
    }

    pub fn get_file_name(&self) -> String {
        self.files[0].filename.clone()
    }

    pub fn get_id(&self) -> String{
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
        self.dependencies.len() > 0
    }

    pub fn get_dependencies(&self) -> &Vec<Dependency> {
        &self.dependencies
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RinthVersions {
    pub versions: Vec<RinthVersion>,
}

impl RinthVersions {
    pub fn get_version(&self, i: usize) -> &RinthVersion {
        &self.versions[i]
    }

    pub fn new() -> RinthVersions {
        RinthVersions { versions: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.versions.len()
    }

    pub fn push(&mut self, version: RinthVersion) {
        self.versions.push(version);
    }

    pub fn last(&self) -> &RinthVersion {
        self.versions.last().unwrap()
    }

    pub fn first(&self) -> &RinthVersion {
        self.versions.first().unwrap()
    }

    pub fn mod_at(&self, i: usize) -> &RinthVersion {
        &self.versions[i]
    }

    pub fn mods(&self) -> &Vec<RinthVersion> {
        &self.versions
    }

    pub fn filter_by(&self, attribute: Attributes, content: &str) -> Vec<RinthVersion> {
        match attribute {
            Attributes::Loader => {
                self.versions
                    .iter()
                    .filter(|x| x.loaders.contains(&content.to_string()))
                    .map(|x| x.clone())
                    .collect::<Vec<RinthVersion>>()
            }

            Attributes::Name => {
                self.versions
                    .iter()
                    .filter(|x| x.name.contains(&content.to_string()))
                    .map(|x| x.clone())
                    .collect::<Vec<RinthVersion>>()
            }

            Attributes::VersionType => {
                self.versions
                    .iter()
                    .filter(|x| x.version_type.contains(&content.to_string()))
                    .map(|x| x.clone())
                    .collect::<Vec<RinthVersion>>()
            }
        }
    }

    pub fn has(&self, id: &str) -> bool{
        self.versions.iter().any(|x| x.project_id == id)
    }
}

impl Default for RinthVersions {
    fn default() -> Self {
        RinthVersions { versions: Vec::new() }
    }
}
    
impl fmt::Display for RinthVersions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut max_width = {
            self
                .versions
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
            write!(
                f,
                "{index:^5}\t{:<max_width$}\t{:^12}\t{:>9}\n",
                version.name, version.version_type, version.downloads
            )?;
        }
        write!(f, "")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]

struct RinthFile {
    pub url: String,
    pub filename: String,
}

/// If a is newer -1, if b is newer 1, if they are the same 0
pub fn compare_versions(a: &RinthVersion, b: &RinthVersion) -> i8{
    let a_version = a.get_versions()[0].clone();
    let b_version = b.get_versions()[0].clone();   
    println!("{} - {}", a_version, b_version);
    for (a_number, b_number) in a_version.split(".").zip(b_version.split(".")) {
        if a_number == b_number {
            continue;
        }

        let a_number = a_number.parse::<u8>().unwrap();
        let b_number = b_number.parse::<u8>().unwrap();

        if a_number > b_number {
            return -1;
        } else {
            return 1;
        }
    }
    return 0;
}