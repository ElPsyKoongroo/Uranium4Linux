use core::fmt;
use serde::{Deserialize, Serialize};

trait Downloadeable {
    fn download() {}
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CurseMod {
    id: u64,
    name: String,
    downloadCount: f64,
}

//
//
// RINTH MODS
//
//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RinthMod {
    project_id: Option<String>,
    title: String,
    //latest_version: String,
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
pub struct RinthVersion {
    id: String,
    project_id: String,
    name: String,
    version_type: String,
    downloads: u64,
    files: Vec<RinthFile>,
    dependencies: Vec<String>,
    loaders: Vec<String>,
}

impl RinthVersion {
    pub fn get_name(&self) -> String {
        self.name.clone()
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
}

impl Default for RinthVersions {
    fn default() -> Self {
        RinthVersions { versions: Vec::new() }
    }
}
    
impl fmt::Display for RinthVersions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut max_width = {
            &self
                .versions
                .iter()
                .map(|x| x.name.clone().len())
                .max()
                .unwrap_or(0)
        };
        let vt_len = "version type".chars().count();

        if max_width < &vt_len {
            max_width = &vt_len;
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
