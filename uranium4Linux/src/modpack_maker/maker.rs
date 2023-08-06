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

/// This struct is responsable for the creation
/// of the modpacks given a minecraft path.
pub struct ModpackMaker {
    path: PathBuf,
    current_state: State,
    hash_filenames: HashFilename,
    mods_states: Vec<ParseState>,
    rinth_pack: RinthModpack,
    raw_mods: Vec<String>,
    cliente: reqwest::Client,
}

impl ModpackMaker {
    pub fn new<I: AsRef<Path>>(path: &I) -> ModpackMaker {
        ModpackMaker {
            path: path.as_ref().to_path_buf(),
            current_state: State::Starting,
            hash_filenames: vec![],
            mods_states: vec![],
            rinth_pack: RinthModpack::new(),
            raw_mods: vec![],
            cliente: reqwest::Client::new(),
        }
    }

    /// This method should be called before calling chunk
    /// or finish method for optimal performance.
    ///
    /// It fetchs the mods from minecraft/mods folder.
    pub fn start(&mut self) {
        self.hash_filenames = self.get_mods();
        self.mods_states = Vec::with_capacity(self.hash_filenames.len());
    }

    /// Can only be used after start method is called.
    ///
    /// If this methods returns Ok(()) it means the modpack
    /// has been made succecsfully
    pub async fn finish(&mut self) -> Result<(), MakerError> {
        loop {
            match self.chunk().await {
                Ok(State::Finish) => return Ok(()),
                Err(e) => return Err(e),
                _ => {}
            }
        }
    }

    /// Returns how many mods are in the minecraft
    /// directory
    pub fn len(&self) -> usize {
        self.hash_filenames.len()
    }

    /// This method will make progress until Ok(State::Finish) is returned
    /// or throw an Err.
    ///
    /// It will return the current State of the process.
    pub async fn chunk(&mut self) -> Result<State, MakerError> {
        self.current_state = match self.current_state {
            State::Starting => {
                if self.hash_filenames.is_empty() {
                    self.hash_filenames = self.get_mods();
                }
                State::Searching
            }
            State::Searching => {
                if self.hash_filenames.is_empty() {
                    State::Checking
                } else {
                    self.search_mods().await;
                    State::Searching
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

                compress_pack("modpack", &self.path, &self.raw_mods)
                    .map_err(|_| MakerError::CantCompress)?;

                std::fs::remove_file(constants::RINTH_JSON)
                    .map_err(|_| MakerError::CantRemoveJSON)?;

                State::Finish
            }
            State::Finish => State::Finish,
        };

        Ok(self.current_state)
    }

    async fn search_mods(&mut self) {
        let threads = N_THREADS();
        let end = if threads > self.hash_filenames.len() {
            self.hash_filenames.len()
        } else {
            threads
        };

        let chunk: HashFilename = self.hash_filenames.drain(0..end).collect();

        // Get rinth_responses
        let mut rinth_responses = Vec::with_capacity(chunk.len());
        let mut pool = AsyncPool::new();

        let reqs = chunk
            .iter()
            .map(|f| tokio::task::spawn(self.cliente.get(maker::ModRinth::hash(&f.0)).send()))
            .collect();
        pool.push_request_vec(reqs);
        rinth_responses.append(&mut pool.start().await);

        let rinth_parses = parse_responses(rinth_responses).await;
        for (file_name, rinth) in chunk.into_iter().zip(rinth_parses.into_iter()) {
            if let Ok(m) = rinth {
                self.mods_states.push(ParseState::Good(m));
            } else {
                self.mods_states.push(ParseState::Raw(file_name.1));
            }
        }
    }

    fn get_mods(&mut self) -> HashFilename {
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
                .to_str()
                .unwrap_or_default()
                .to_owned();
            hashes_names.push((mod_hash, file_name));
        }

        hashes_names
    }
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
