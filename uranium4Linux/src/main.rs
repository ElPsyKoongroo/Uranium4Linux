#![allow(non_snake_case)]
mod checker;
mod code_functions;
mod easy_input;
mod hashes;
mod modpack_downloader;
mod modpack_maker;
mod searcher;
mod variables;
mod zipper;

use code_functions::{fix_path, get_bool_element, get_parse_element, update, N_THREADS};
use modpack_downloader::curse_downloader::curse_modpack_downloader;
use modpack_downloader::rinth_downloader::download_rinth_pack;
use modpack_maker::maker::make_modpack;
use searcher::rinth::SearchType;
use std::env;
use variables::constants::{
    CURSE_FLAG, DEFAULT_NTHREADS, DOWNLOAD, FILE_FLAG, FOR, HELP, HELP_MSG, MAKE, MOD, NTHREADS,
    PROJECT, QUERY, REQUEST, RESOURCEPACKS, RINTH_FLAG, ROOT_FLAG, THREADS_FLAG, VERSION, VERSIONS,
};
use zip::result::ZipError;
use zipper::pack_unzipper::unzip_pack;

fn request_arg_parser(args: &[String]) -> Option<SearchType> {
    match args.iter().position(|f| f == REQUEST) {
        Some(index) => match args[index + 1].as_str() {
            QUERY => Some(SearchType::QUERRY(args[index + 2].clone())),
            FOR => Some(SearchType::FOR(
                args[index + 2].parse().unwrap(),
                args[index + 3].parse().unwrap(),
            )),
            VERSION => Some(SearchType::VERSION(args[index + 1].clone())),
            VERSIONS => Some(SearchType::VERSIONS(args[index + 1].clone())),
            MOD => Some(SearchType::MOD(args[index + 1].clone())),
            PROJECT => Some(SearchType::PROJECT(args[index + 1].clone())),
            RESOURCEPACKS => Some(SearchType::RESOURCEPACKS(
                args[index + 2].parse().unwrap(),
                args[index + 3].parse().unwrap(),
            )),

            _ => panic!("Invalid request type !"),
        },
        None => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), ZipError> {
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

    match args[1].as_str() {
        DOWNLOAD => {
            if curse_pack {
                curse_modpack_downloader(&file_path, &destination_path).await;
            } else if rinth_pack {
                download_rinth_pack(&file_path, &destination_path).await;
            } else {
                unzip_pack(&file_path, &destination_path, N_THREADS())
                    .await
                    .unwrap();
            }
        }
        "-u" => update(args[2].as_str()).await,
        MAKE => make_modpack(&file_path, N_THREADS()).await,
        REQUEST => searcher::rinth::search(request_arg_parser(&args).unwrap()).await,
        HELP => println!("{}", HELP_MSG),
        _ => println!("Invalid arguments"),
    }
    Ok(())
}
