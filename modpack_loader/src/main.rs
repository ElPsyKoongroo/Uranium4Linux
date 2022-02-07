#![allow(non_snake_case)]


mod modpack_loader;
use modpack_loader::loader::*;
use modpack_loader::modpack_struct::*;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::env;

#[allow(unused_must_use)]
fn new_modpck(){
    let modPack = ModPack{
        count: 0,
        name: "Pruebita".to_string(),
        version: "1.0.0".to_string(),
        author: "El sergio".to_string(),
        mods: vec![
            Mods::new(
                String::from("Sodium"),
                String::from("https://cdn.modrinth.com/data/AANobbMI/versions/mc1.18.1-0.4.0-alpha6/sodium-fabric-mc1.18.1-0.4.0-alpha6+build.14.jar"),
                String::from("sodium-fabric-mc1.18.1-0.4.0-alpha6+build.14.jar")
            ),
            Mods::new(
                String::from("Iris"),
                String::from("https://cdn.modrinth.com/data/YL57xq9U/versions/1.18.x/v1.2.0/iris-mc1.18.1-1.2.0-pre.jar"),
                String::from("iris-mc1.18.1-1.2.0-pre.jar")
            ),
            Mods::new(
                String::from("Hydrogen"),
                String::from("https://cdn.modrinth.com/data/AZomiSrC/versions/mc1.17.1-0.3.1/hydrogen-fabric-mc1.17.1-0.3.jar"),
                String::from("Hydrogen.jar")
            ),
            Mods::new(
                String::from("Lazy"),
                String::from("https://cdn.modrinth.com/data/hvFnDODi/versions/0.1.2/lazydfu-0.1.2.jar"),
                String::from("Lazy.jar")
            ),
            Mods::new(
                String::from("Lithium"),
                String::from("https://cdn.modrinth.com/data/gvQqBUqZ/versions/mc1.18.1-0.7.7/lithium-fabric-mc1.18.1-0.7.7.jar"),
                String::from("Lithium.jar")
            )
        ]
    };
    let mut file = File::create("mi_modpack.modpck").unwrap();
    let a = serde_json::to_string_pretty(&modPack).unwrap();
    
    file.write_all(a.as_bytes());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    new_modpck();
    let args: Vec<String> = env::args().collect();
    let testing = true;
    println!("{}", args[1]);
    match args.len() >= 2{
        true => {
            // mi_modpack.modpck
            let mut modpack_loader = ModPackDownloader::new();
            modpack_loader.load_pack(args[1].to_string());
            if testing{
                for _ in 0..10{
                    modpack_loader.start().await.unwrap();
                    println!("\n\n");
                }
            } else { 
                modpack_loader.start().await.unwrap();
            }
            Ok(())
        }

        _ => {
            Err(Box::<dyn Error>::from("Bad modpack!"))
        }
    }
}
