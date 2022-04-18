use std::{path::Path, error::Error};
use crate::modpack_loader::loader::ModPackDownloader;





pub async fn download_modpack(modpack: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Err(Box::<dyn Error>::from(format!(
            "{} is not a valid path !",
            path
        )));
    };
    let mut modpack_loader = ModPackDownloader::new();
    modpack_loader.set_path(String::from(path));
    modpack_loader.load_pack(modpack);
    modpack_loader.start().await?;
    println!("\n\n");
    Ok(())
}



/*
            DEPRECATED
||||||||||||||||||||||||||||||||||
VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV




pub enum CODES {
    Exit,
    ModSelected,
    PageSelected,
    SetPath,
    ParseError,
    MakeModPack,
}

#[allow(dead_code)]
pub struct Properties {
    limit: u32,
    page: u32,
    offset: u32,
    selected_mod: usize,
    path: String,
}

impl Properties {
    pub fn new() -> Properties {
        Properties {
            limit: 20,
            page: 0,
            offset: 0,
            selected_mod: 0,
            path: String::from("./"),
        }
    }

    pub fn get_limit(&self) -> u32 {
        self.limit
    }

    pub fn get_page(&self) -> u32 {
        self.page
    }

    #[allow(dead_code)]
    pub fn get_offset(&self) -> u32 {
        self.offset
    }

    pub fn get_selected_mod(&self) -> usize {
        self.selected_mod
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    #[allow(dead_code)]
    pub fn set_limit(&mut self, limit: u32) {
        self.limit = limit;
    }

    pub fn set_page(&mut self, page: u32) {
        self.page = page;
    }

    #[allow(dead_code)]
    pub fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    pub fn set_selected_mod(&mut self, selected_mod: usize) {
        self.selected_mod = selected_mod;
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
}

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub async fn download_mod(
    minecraft_mod_versions: &RinthVersions,
    requester: &Requester,
    path: &String,
) -> Result<i32, Box<dyn std::error::Error>> {
    let index: i32 = input("Select the desiere version: ", -1);

    match index.is_negative() {
        false => {
            let response = requester
                .get(
                    minecraft_mod_versions
                        .get_version(index as usize)
                        .get_file_url(),
                )
                .await?;
            let content = response.bytes().await?;
            let path = path.clone().add(
                minecraft_mod_versions
                    .get_version(index as usize)
                    .get_file_name()
                    .as_str(),
            );
            tokio::fs::write(path, content).await?;

            Ok(index)
        }

        true => Err(Box::new(Error::new(ErrorKind::Other, "Bad input!"))),
    }
}

pub fn exits_path(p: &Path) -> bool {
    match Path::new(p).is_dir() {
        true => return true,

        false => {
            eprintln!("This is not a valid directory!!");
            false
        }
    }
}

pub fn set_path() -> String {
    let temp = input("New path: ", String::from("./"));
    let path = temp.as_str();

    match exits_path(Path::new(path)) {
        true => return path.to_string(),
        false => String::from("./")
    }
}

fn get_sha1_from_file(file_path: &String) -> String {
    let mut hasher = Sha1::new();
    let mut file = fs::File::open(file_path).unwrap();
    let metadata = fs::metadata(file_path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");

    hasher.update(buffer);
    let temp = hasher.finalize().to_vec();
    let hash = temp.encode_hex::<String>();
    hash
}

pub fn get_mods(mods_path: &Path) -> Option<Vec<(String, String)>> {
    let mut names: Vec<(String, String)> = Vec::new();
    let mods;

    if !mods_path.is_dir() {return None;}
    
    match read_dir(mods_path) {
        Ok(e) => mods = e,
        Err(error) => {
            eprintln!("Error reading the directore: {}", error);
            return None
        }
    }

    for mmod in mods {
        get_sha(mods_path, mmod.unwrap(), &mut names);
    }

    Some(names)
}

fn get_sha(path: &Path, mod_dir: fs::DirEntry, names_vec: &mut Vec<(String, String)>) {
    let file_name = mod_dir.file_name().into_string().unwrap();
    let file_path = { path.join(&file_name).to_str().unwrap().to_string() };
    let hash = get_sha1_from_file(&file_path);
    names_vec.push((hash, file_name));
}

pub async fn search_mods_for_modpack(requester: &mut Requester, hash_filename: Vec<(String, String)>, responses: &mut RinthVersions) {
    for item in hash_filename {
        let response = {
            let request = requester.get(maker::ModRinth::hash(&item.0)).await.unwrap();
            check(
                request.json::<RinthVersion>().await,
                false,
                true,
                format!("Mod {} was not found !", &item.1).as_str(),
            )
        };
        match response {
            Some(e) => responses.push(e),
            None => {}
        }
    }
}

pub fn one_input(input: String) -> CODES {
    match input.as_str() {
        "path" => CODES::SetPath,
        "make" => CODES::MakeModPack,
        "exit" => CODES::Exit,
        _ => CODES::ParseError,
    }
}

pub fn two_inputs(opt: String, value: &str, properties: &mut Properties) -> CODES {
    match opt.as_str() {
        "mod" => {
            properties.set_selected_mod(value.parse::<usize>().unwrap());
            return CODES::ModSelected;
        }
        "page" => {
            properties.set_page(value.parse::<u32>().unwrap());
            return CODES::PageSelected;
        }

        _ => return CODES::ParseError,
    }
}

pub fn menu(properties: &mut Properties) -> CODES {
    println!("{}", MENU);

    let user_input: Vec<String> = {
        let aux = easy_input::input("Chose an option: ", String::from(" "));
        aux.split(" ").map(|x| x.to_string()).collect()
    };

    match user_input.len() {
        1 => one_input(user_input[0].clone()),
        2 => two_inputs(user_input[0].clone(), user_input[1].as_str(), properties),
        _ => CODES::ParseError,
    }
}

pub async fn page_selection(
    pages: &mut HashMap<u32, RinthResponse>,
    properties: &mut Properties,
    requester: &mut Requester,
    actual_page: &mut RinthResponse,
) {
    if !pages.contains_key(&properties.get_page()) {
        let resp = requester
            .get(maker::ModRinth::search_for(
                properties.get_limit(),
                properties.get_page() * 20,
            ))
            .await
            .unwrap();
        *actual_page = check(
            resp.json::<RinthResponse>().await,
            true,
            true,
            "No page found",
        )
        .unwrap_or_default();
        if actual_page.len() == 0 {
            println!("This page is empty, nothing here !");
        } else {
            pages.insert(properties.get_page(), actual_page.clone());
        }
    } else {
        *actual_page = pages.get(&properties.get_page()).unwrap().clone();
    }
}

pub async fn mod_selection(
    properties: &mut Properties,
    requester: &mut Requester,
    actual_page: &mut RinthResponse,
) {
    let actual_mod = &actual_page.hits[properties.get_selected_mod()];
    let version_resp = requester
        .get(maker::ModRinth::mod_versions(actual_mod))
        .await
        .unwrap();
    println!(
        "\n\n{}\n{}",
        actual_mod.get_title().to_uppercase(),
        actual_mod.get_description()
    );
    let minecraft_mod = check(
        version_resp.json::<RinthVersions>().await,
        true,
        true,
        "No mod found",
    )
    .unwrap();
    println!("{}", minecraft_mod);
    match download_mod(&minecraft_mod, &requester, &properties.get_path()).await {
        Ok(_) => {}
        Err(e) => println!("Runtime Error => {}", e.to_string()),
    }
    let _ = easy_input::input::<String>("Press enter to continue...", String::from(" "));
}

pub async fn make_modpack(requester: &mut Requester) {
    let input = easy_input::input("Path: ", String::from("-"));
    let path = Path::new(input.as_str());
    let hash_filename = get_mods(path).unwrap();
    let mut responses: RinthVersions = RinthVersions::new();
    search_mods_for_modpack(requester, hash_filename, &mut responses).await;
    
    let mp_name = easy_input::input("Modpack name: ", String::from("Modpack.mm"));
    let mp_version = easy_input::input("Modpack version: ", String::from("1.0"));
    let mp_author = easy_input::input("Modpack author: ", String::from("Anonimous"));
    let mp =
        mine_data_strutcs::modpack_struct::ModPack::modpack_from_RinthVers(mp_name, mp_version, mp_author, responses);
    mp.write_mod_pack();

    let _ = easy_input::input("Press enter to continue...", 0);
}

*/