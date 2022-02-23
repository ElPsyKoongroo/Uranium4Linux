#![allow(unused_imports)]
#![allow(non_snake_case)]
use reqwest::header::HeaderMap;
use serde_json::to_string;
use tokio::fs::read_dir;
use std::collections::HashMap;
use std::path::Path;
use requester::requester::load_headers::*;
use requester::requester::request_maker::*;

mod checker;
use crate::checker::check;

use modpack_loader::modpack_loader::*;
use minecraft_mod::minecraft_mod::*;
use minecraft_mod::responses::*;


mod variables;
use crate::variables::*;

mod code_functions;
use crate::code_functions::*;

mod easy_input;
use crate::easy_input::input;

mod url_maker;
use crate::url_maker::*;


fn menu(properties: &mut Properties) -> CODES {
    println!(
        "
        mod  + <number>\n
        page + <number>\n
        path \n
        make \n        
        ",
    );

    let user_input = easy_input::input("Chose an option: ", " ".to_string());
    let mut aux = user_input.split(" ");

    let option = aux.next().unwrap();
    if option.to_lowercase() == "exit" {return CODES::Exit}
    let parsed_value;
    let value;
    if option != "path" && option != "make"{
        value = aux.next().unwrap();
        match value.parse::<u32>() {
            Ok(a) => parsed_value = a,
            Err(_) => {
                parsed_value = 0;
            }
        }
    } else {
        parsed_value = 0;
    }

    match option {
        "mod" => {
            properties.selected_mod = parsed_value as usize;
            return CODES::ModSelected;
        }
        "page" => {
            properties.page = parsed_value;
            return CODES::PageSelected;
        }

        "path" => {
            return CODES::SetPath;
        }

        "make" => {
            return CODES::MakeModPack;
        }
        _ => {
            println!("Ooops!");
            return CODES::ParseError;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let mut requester = Requester::new();
    requester.set_headers(headers.clone());

    let mut pages: HashMap<u32, RinthResponse> = HashMap::new();
    let mut actual_page: RinthResponse; // = RinthResponse::new();

    let input = easy_input::input_string("Chose mod repo (RINTH/CURSE): ", "RINTH".to_string());
    let repo = input.as_str();
    println!("Chosen repo: {}", repo);
    load_headers(repo.to_uppercase().as_str(), &mut headers);

    let mut properties = Properties {
        limit: 20,
        page: 0,
        offset: 0,
        selected_mod: 0,
        path: String::from("./")
    };

    {
        let resp = requester
            .get(maker::ModRinth::search_for(properties.limit, 0 * 20))
            .await?;
        actual_page = resp.json::<RinthResponse>().await?;
        pages.insert(properties.page, actual_page.clone());
    }



    loop {
        actual_page.show();
        match menu(&mut properties) {
            CODES::PageSelected => {
                if !pages.contains_key(&properties.page) {
                    let resp = requester
                        .get(maker::ModRinth::search_for(
                            properties.limit,
                            properties.page * 20,
                        ))
                        .await?;
                    actual_page = check(resp.json::<RinthResponse>().await).unwrap_or_default();
                    if actual_page.len() == 0 {
                        println!("This page is empty, nothing here !")
                    } else {
                        pages.insert(properties.page, actual_page.clone());
                    }
                } else {
                    actual_page = pages.get(&properties.page).unwrap().clone();
                }
            }

            CODES::ModSelected => {
                let actual_mod = &actual_page.hits[properties.selected_mod];
                let resp = requester
                    .get(maker::ModRinth::mod_versions(
                        actual_mod,
                    ))
                    .await?;
                println!("\n\n{}\n{}", actual_mod.get_title().to_uppercase()  ,actual_mod.get_description());
                let m_versions = check(resp.json::<Vec<RinthVersion>>().await).unwrap_or_default();
                let minecraft_mod = RinthVersions {versions: m_versions};
                println!("{}", minecraft_mod);
                match download_mod(&minecraft_mod, &requester, &properties.path).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Runtime Error => {}", e.to_string());
                    }
                }
                let _ = easy_input::input::<String>("Press enter to continue...", " ".to_string());
            }

            CODES::SetPath => {
                properties.path = set_path();
            }

            CODES::MakeModPack => {
                let input = easy_input::input("Path: ", String::from("-"));
                let path = Path::new(input.as_str());
                let a = get_mods(path).unwrap();
                let mut responses: Vec<RinthVersion> = Vec::new();
                for item in a{
                    let response = {
                        let request= requester.get(
                            maker::ModRinth::hash(&item.0)
                        ).await?;
                        check(request.json::<RinthVersion>().await)
                    };
                    match response{
                        Some(e) =>  responses.push(e),
                        None => {}
                    }
                    
                }                
                let mp = modpack_struct::ModPack::modpack_from_RinthVers(responses);
                mp.write_mod_pack();

                let _ = easy_input::input("Press enter to continue...", 0);
            }

            CODES::Exit => break,

            _ => break,
        }
        clear_screen();
    }

    Ok(())
}
