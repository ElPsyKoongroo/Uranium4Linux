#![allow(non_snake_case)]

use std::env;
use zip::result::ZipError;
use code_functions::{fix_path, get_bool_element, get_parse_element, N_THREADS, update};
use downloaders::curse_downloader::curse_modpack_downloader;
use downloaders::minecraft_downloader;
use downloaders::rinth_downloader::download_rinth_pack;
use modpack_maker::maker::make_modpack;
use searcher::rinth::SearchType;
use variables::constants::*;

mod checker;
mod code_functions;
mod easy_input;
mod hashes;
mod downloaders;
mod modpack_maker;
mod searcher;
mod variables;
mod zipper;

fn request_arg_parser(args: &[String]) -> Option<SearchType> {
    match args.iter().position(|f| f == REQUEST) {
        Some(index) => match args[index + 1].as_str() {
            QUERY => Some(SearchType::QUERY(args[index + 2].clone())),
            FOR => Some(SearchType::FOR(
                args[index + 2].parse().expect(&format!("{} not a number", args[index + 2])),
                args[index + 3].parse().expect(&format!("{} not a number", args[index + 3])),
            )),
            VERSION => Some(SearchType::VERSION(args[index + 1].clone())),
            VERSIONS => Some(SearchType::VERSIONS(args[index + 1].clone())),
            MOD => Some(SearchType::MOD(args[index + 1].clone())),
            PROJECT => Some(SearchType::PROJECT(args[index + 1].clone())),
            RESOURCEPACKS => Some(SearchType::RESOURCEPACKS(
                args[index + 2].parse().expect(&format!("{} not a number", args[index + 3])),
                args[index + 3].parse().expect(&format!("{} not a number", args[index + 3])),
            )),

            _ => panic!("Invalid request type !"),
        },
        None => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), ZipError> {
    #[cfg(debug_assertions)] println!("Debug enable"); 

    let args: Vec<String> = env::args().collect();

    {
        let mut aux = NTHREADS.write().unwrap();
        *aux = get_parse_element(&args, THREADS_FLAG).unwrap_or(DEFAULT_NTHREADS);
    }

    let file_path = get_parse_element(&args, FILE_FLAG).unwrap_or_else(|| "".to_owned());
    let mut destination_path = get_parse_element(&args, ROOT_FLAG).unwrap_or_else(|| "".to_owned());
    let curse_pack = get_bool_element(&args, CURSE_FLAG);
    let rinth_pack = get_bool_element(&args, RINTH_FLAG);

    destination_path = fix_path(&destination_path);

    // TODO! Replace manual argument parse with CLAP
    match args[1].as_str() {
        DOWNLOAD | LONG_DOWNLOAD => {
            if rinth_pack {
                download_rinth_pack(&file_path, &destination_path).await;
            } else if curse_pack {
                curse_modpack_downloader(&file_path, &destination_path).await;
            } else {
                println!("No repo found!");
            }
        }
        LONG_UPDATE => update(&file_path).await,
        LONG_MAKE => make_modpack(&file_path, N_THREADS()).await,
        LONG_REQUEST => searcher::rinth::search(request_arg_parser(&args).expect("Wrong request type")).await,
        LONG_INSTACIATE => minecraft_downloader::donwload_minecraft(&destination_path).await.unwrap(),
        HELP | LONG_HELP => println!("{}", HELP_MSG),
        _ => println!("Invalid arguments"),
    }
    Ok(())
}
