#![allow(non_snake_case)]
mod checker;
mod code_functions;
mod easy_input;
mod modpack_downloader;
mod modpack_maker;
mod variables;
mod zipper;

use code_functions::{download_modpack, update};
use modpack_maker::maker::make_modpack;
use std::env;
use variables::constants::*;
use zipper::pack_unzipper::unzip_pack;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "-d" => return download_modpack(&args[2], &args[3]).await,
        "-u" => update(args[2].as_str()).await,
        "-m" => make_modpack(args[2].as_str()).await,
        "-t" => unzip_pack(&args[2]).unwrap(),
        "-h" => println!("{}", HELP),
        _    => println!("{}", "Invalid arguments")
    }
    Ok(())
}
