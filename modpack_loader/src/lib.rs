
pub mod modpack_loader; 
use std::error::Error;

use crate::modpack_loader::loader::*;

#[allow(dead_code)]
async fn download_modpack(modpack: String) -> Result<(), Box<dyn std::error::Error>>{
    let testing = true;
    // TODO
    match true{
        true => {
            let mut modpack_loader = ModPackDownloader::new();
            modpack_loader.set_path(String::from("U:\\Programacion\\rust\\Uranium4Linux\\temp\\"));
            modpack_loader.load_pack(modpack);
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


#[tokio::test]
async fn my_test() {
    use crate::modpack_loader::loader::ModPackDownloader;

    let path = "U:\\Programacion\\rust\\Uranium4Linux\\mi_modpack.modpck";
    let testing = true;
    match true{
        true => {
            let mut modpack_loader = ModPackDownloader::new();
            modpack_loader.set_path(String::from("U:\\Programacion\\rust\\Uranium4Linux\\temp\\"));
            modpack_loader.load_pack(path.to_string());
            if testing{
                for _ in 0..10{
                    modpack_loader.start().await.unwrap();
                    println!("\n\n");
                }
            } else { 
                modpack_loader.start().await.unwrap();
            }
        }

        _ => {
            println!("Bad modpack")
        }
    }
    assert!(true);
}