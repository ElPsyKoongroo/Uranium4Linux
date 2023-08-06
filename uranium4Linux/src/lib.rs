#![forbid(unsafe_code)]

use std::{io::Write, path::Path};

use downloaders::rinth_downloader::*;
use error::{ModpackError, MakerError};
use modpack_maker::maker::{ModpackMaker, State};
use searcher::rinth::SearchType;
use variables::constants::*;

mod checker;
mod code_functions;
pub mod downloaders;
pub mod error;
mod hashes;
pub mod modpack_maker;
pub mod searcher;
pub mod variables;
pub mod zipper;



/// This function will make a Modpack from the 
/// given path.
pub async fn make_modpack<I: AsRef<Path>>(minecraft_path: I) -> Result<(), MakerError>  {
    let mut maker = ModpackMaker::new(&minecraft_path);
    maker.start();
    let mut i = 0;
    loop {
        match maker.chunk().await {
            Ok(State::Finish) => return Ok(()),
            Err(e) => return Err(e),
            _ => {
                println!("{}", i);
                i += 1;
            }
        }
    }
    
    //ModpackMaker::make(&minecraft_path).await
}

/// This function will download the modpack specified by `file_path`
/// into `destination_path`
///
/// If there is no mods and/or config folder inside `destination_path` then they
/// will be created.
pub async fn rinth_pack_download<I: AsRef<Path>>(
    file_path: I,
    destination_path: I,
) -> Result<(), ModpackError> {
    let mut rinth_downloader = RinthDownloader::new(&file_path, &destination_path)?;
    rinth_downloader.start().await;
    let total = rinth_downloader.len() * 2;
    let mut i = 1;

    loop {
        let _ = std::io::stdout().flush();
        if rinth_downloader.chunk().await.is_some() {
            print!("\r{} / {}      ", i, total);
            i += 1;
        } else {
            println!();
            return Ok(());
        }
    }
}

pub fn set_threads(t: usize) {
    let mut aux = NTHREADS.write().unwrap();
    *aux = t;
}

pub fn request_arg_parser(args: &[String]) -> Option<searcher::rinth::SearchType> {
    match args
        .iter()
        .position(|f| f == SHORT_REQUEST || f == LONG_REQUEST)
    {
        Some(index) => match args[index + 1].as_str() {
            QUERY => Some(SearchType::QUERY(args[index + 2].clone())),
            FOR => Some(SearchType::FOR(
                args[index + 2]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 2])),
                args[index + 3]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 3])),
            )),
            VERSION => Some(SearchType::VERSION(args[index + 1].clone())),
            VERSIONS => Some(SearchType::VERSIONS(args[index + 1].clone())),
            MOD => Some(SearchType::MOD(args[index + 1].clone())),
            PROJECT => Some(SearchType::PROJECT(args[index + 1].clone())),
            RESOURCEPACKS => Some(SearchType::RESOURCEPACKS(
                args[index + 2]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 3])),
                args[index + 3]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 3])),
            )),

            MODPACKS => Some(SearchType::MODPACKS(
                args[index + 2]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 3])),
                args[index + 3]
                    .parse()
                    .unwrap_or_else(|_| panic!("{} not a number", args[index + 3])),
            )),

            _ => panic!("Invalid request type !"),
        },
        None => None,
    }
}
