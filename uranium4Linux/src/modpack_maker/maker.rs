use core::panic;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use futures::future::join_all;

use mine_data_strutcs::{
    rinth::{rinth_mods::RinthVersion, rinth_packs::RinthModpack},
    url_maker::maker,
};
use requester::async_pool::AsyncPool;

use crate::{
    code_functions::N_THREADS, error::MakerError, hashes::rinth_hash, variables::constants,
    zipper::pack_zipper::compress_pack,
};

type HashFilename = Vec<(String, String)>;

/// Good -> Means Uranium found the mod
/// Raw  -> Means the mod need to be added raw
enum ParseState {
    Good(RinthVersion),
    Raw(String),
}

#[derive(Clone, Copy)]
pub enum State {
    Starting,
    Searching,
    Checking,
    Writing,
    Finish,
}

pub struct ModpackMaker<'a> {
    path: &'a Path,
    current_state: State,
    hash_filenames: HashFilename,
    mods_states: Vec<ParseState>,
    rinth_pack: RinthModpack,
    raw_mods: Vec<String>,
}

impl<'a> ModpackMaker<'a> {
    pub fn new<I: AsRef<Path>>(path: &'a I) -> ModpackMaker<'a> {
        ModpackMaker {
            path: path.as_ref(),
            current_state: State::Starting,
            hash_filenames: vec![],
            mods_states: vec![],
            rinth_pack: RinthModpack::new(),
            raw_mods: vec![],
        }
    }

    pub async fn start(&mut self) {
        self.hash_filenames = self.get_mods();
    }

    async fn search_mods(&mut self) {
        let cliente = reqwest::Client::new();

        let end = if N_THREADS() > self.hash_filenames.len() {
            self.hash_filenames.len()
        } else {
            N_THREADS()
        };

        let chunk: Vec<(String, String)> = self.hash_filenames.drain(0..end).collect();
        self.mods_states = Vec::with_capacity(self.hash_filenames.len());

        // Get rinth_responses
        let mut rinth_responses = Vec::with_capacity(chunk.len());
        let mut pool = AsyncPool::new();

        let reqs = chunk
            .iter()
            .map(|f| tokio::task::spawn(cliente.get(maker::ModRinth::hash(&f.0)).send()))
            .collect();
        pool.push_request_vec(reqs);
        rinth_responses.append(&mut pool.start().await);

        let rinth_parses = parse_responses(rinth_responses).await;
        for (i, rinth) in rinth_parses.into_iter().enumerate() {
            if let Ok(m) = rinth {
                self.mods_states.push(ParseState::Good(m));
            } else {
                self.mods_states
                    .push(ParseState::Raw(chunk[i].1.clone()));
            }
        }

        // Get rinth parses
    }

    pub fn get_mods(&mut self) -> HashFilename {
        assert!(self.path.is_dir(), "{:?} is not a dir", self.path);

        let mods_path = self.path.join("mods/");

        let mods = match read_dir(&mods_path) {
            Ok(e) => e
                .into_iter()
                .map(|f| f.unwrap().path())
                .collect::<Vec<PathBuf>>(),
            Err(error) => {
                eprintln!("Error reading the directory: {}", error);
                panic!("")
            }
        };

        let mut hashes_names = Vec::with_capacity(mods.len());

        // Push all the (has, file_name) to the vector
        for path in mods {
            let mod_hash = rinth_hash(path.as_path());
            let file_name = path
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap_or_default();
            hashes_names.push((mod_hash, file_name));
        }

        hashes_names
    }

    pub async fn chunk(&mut self) -> Result<State, MakerError> {
        self.current_state = match self.current_state {
            State::Starting => {
                self.get_mods();
                State::Searching
            }
            State::Searching => {
                if !self.hash_filenames.is_empty() {
                    self.search_mods().await;
                    State::Searching
                } else {
                    State::Checking
                }
            }
            State::Checking => {
                for rinth_mod in &self.mods_states {
                    match rinth_mod {
                        ParseState::Good(m) => self.rinth_pack.add_mod(m.clone().into()),
                        ParseState::Raw(file_name) => self.raw_mods.push(file_name.clone()),
                    }
                }
                State::Writing
            }
            State::Writing => {
                self.rinth_pack.write_mod_pack_with_name();

                compress_pack("modpack", self.path, &self.raw_mods)
                    .map_err(|_| MakerError::CantCompress)?;

                std::fs::remove_file(constants::RINTH_JSON)
                    .map_err(|_| MakerError::CantRemoveJSON)?;

                State::Finish
            }
            State::Finish => {
                State::Finish
            }
        };

        Ok(self.current_state)
    }

    pub async fn make<I>(path: &I) -> Result<(), MakerError>
    where
        I: AsRef<Path>,
    {
        let hash_filename = get_mods(Path::new(path.as_ref()));

        let mods_states = search_mods_for_modpack(&hash_filename, N_THREADS()).await;

        let mp_name = "modpack".to_owned();

        let mut rinth_pack = RinthModpack::new();
        let mut raw_mods = Vec::new();
        for rinth_mod in mods_states {
            match rinth_mod {
                ParseState::Good(m) => rinth_pack.add_mod(m.into()),
                ParseState::Raw(file_name) => raw_mods.push(file_name),
            }
        }

        rinth_pack.write_mod_pack_with_name();

        compress_pack(&mp_name, path.as_ref(), &raw_mods).map_err(|_| MakerError::CantCompress)?;

        std::fs::remove_file(constants::RINTH_JSON).map_err(|_| MakerError::CantRemoveJSON)?;
        Ok(())
    }
}

pub async fn make_modpack(path: &str, n_threads: usize) {
    let hash_filename = get_mods(Path::new(path));

    let mods_states = search_mods_for_modpack(&hash_filename, n_threads).await;

    let mp_name = "modpack".to_owned();

    let mut rinth_pack = RinthModpack::new();
    let mut raw_mods = Vec::new();
    for rinth_mod in mods_states {
        match rinth_mod {
            ParseState::Good(m) => rinth_pack.add_mod(m.into()),
            ParseState::Raw(file_name) => raw_mods.push(file_name),
        }
    }

    rinth_pack.write_mod_pack_with_name();

    let path = PathBuf::from(path);
    compress_pack(&mp_name, path.as_path(), &raw_mods).unwrap();

    std::fs::remove_file(constants::RINTH_JSON).unwrap();
}

fn get_mods(minecraft_path: &Path) -> Vec<(String, String)> {
    let mut hashes_names = Vec::new();
    assert!(minecraft_path.is_dir(), "{:?} is not a dir", minecraft_path);

    let mods_path = minecraft_path.join("mods/");

    let mods = match read_dir(&mods_path) {
        Ok(e) => e
            .into_iter()
            .map(|f| f.unwrap().path())
            .collect::<Vec<PathBuf>>(),
        Err(error) => {
            eprintln!("Error reading the directory: {}", error);
            panic!("")
        }
    };

    // Push all the (has, file_name) to the vector
    for path in mods {
        let mod_hash = rinth_hash(path.as_path());
        let file_name = path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap_or_default();
        hashes_names.push((mod_hash, file_name));
    }

    hashes_names
}

/// Search the mods in mods/ in `RinthAPI` by hash,
/// returning a vector of ParseState
async fn search_mods_for_modpack(
    hash_filename: &[(String, String)],
    n_threads: usize,
) -> Vec<ParseState> {
    search_mod(hash_filename, n_threads).await
}

/// * `item` --> \[hashes, file_names]
async fn search_mod(item: &[(String, String)], n_threads: usize) -> Vec<ParseState> {
    let n_mods = item.len();
    let cliente = reqwest::Client::new();
    let chunks = item.chunks(n_threads).collect::<Vec<&[(String, String)]>>();

    // Get rinth_responses
    let mut rinth_responses = Vec::with_capacity(n_mods);
    for chunk in chunks {
        let mut pool = AsyncPool::new();
        let reqs = chunk
            .iter()
            .map(|f| tokio::task::spawn(cliente.get(maker::ModRinth::hash(&f.0)).send()))
            .collect();
        pool.push_request_vec(reqs);
        // pool.start().await;
        rinth_responses.append(&mut pool.start().await);
    }

    // Get rinth parses
    let rinth_parses = parse_responses(rinth_responses).await;
    let mut mods_states = Vec::with_capacity(n_mods);
    for (i, rinth) in rinth_parses.into_iter().enumerate() {
        if let Ok(m) = rinth {
            mods_states.push(ParseState::Good(m));
        } else {
            mods_states.push(ParseState::Raw(item[i].1.clone()));
        }
    }
    mods_states
}

async fn parse_responses(
    responses: Vec<Result<reqwest::Response, reqwest::Error>>,
) -> Vec<Result<RinthVersion, reqwest::Error>> {
    join_all(
        responses
            .into_iter()
            .map(|request| request.unwrap().json::<RinthVersion>()),
    )
    .await
}
