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


/*
    TODO: 
        - If a mod cant be found on Rinth, add the .jar to the modpack.
        - Unzip de modpack in minecraft folder.
        - Add '.' to the modpack name. (now it writes Name + "zip" instead of Name + ".zip")

*/

#[tokio::main]
async fn main() -> Result<(), usize> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "-d" => return download_modpack(&args[2], &args[3]).await,
        "-u" => update(args[2].as_str()).await,
        "-m" => make_modpack(args[2].as_str()).await,
        "-t" => unzip_pack(&args[2], &args[3]).await.unwrap(),
        "-h" => println!("{}", HELP),
        _    => println!("{}", "Invalid arguments")
    }
    Ok(())
}
