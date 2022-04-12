#![allow(non_snake_case)]
mod modpack_loader;
mod functions;
use functions::{download_modpack, update};
use std::env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "-d" => {
            let modpack = args[2].clone();
            let path = args[3].clone();
            return download_modpack(modpack, path).await
        }
        "-u" => {
            let path = args[2].clone();
            update(path).await;
        }
        _ => {
            println!("{}", "Invalid arguments");
        }
    }
    Ok(())
}
