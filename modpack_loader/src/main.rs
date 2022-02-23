#![allow(non_snake_case)]


mod modpack_loader;
use std::error::Error;
use std::env;

use crate::modpack_loader::loader::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    match args.len() >= 2{
        true => {
            let mut modpack_loader = ModPackDownloader::new();
            modpack_loader.set_path(String::from("U:\\Programacion\\rust\\Uranium4Linux\\temp\\"));
            modpack_loader.load_pack(args[1].to_string());
            modpack_loader.start().await.unwrap();
            println!("\n\n");
            Ok(())
        }

        _ => {
            Err(Box::<dyn Error>::from("Bad modpack!"))
        }
    }
}
