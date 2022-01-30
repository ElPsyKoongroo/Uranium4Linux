#![allow(unused_imports)]
use reqwest::header::HeaderMap;
use serde_json::to_string;
use std::collections::HashMap;

mod minecraft;
use crate::minecraft::minecraft_mod::*;
use crate::minecraft::responses::*;

mod requester;
use crate::requester::load_headers::*;
use crate::requester::request_maker::*;

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
        ",
    );

    let user_input = easy_input::input("Chose an option: ", " ".to_string());
    let mut aux = user_input.split(" ");

    let option = aux.next().unwrap();
    let value = aux.next().unwrap();
    let parsed_value;

    match value.parse::<u32>() {
        Ok(a) => parsed_value = a,
        Err(_) => {
            println!("Bad input error!!");
            return CODES::ParseError;
        }
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
        limit: 0,
        page: 0,
        offset: 0,
        selected_mod: 0,
    };
    properties.limit = 20; //easy_input::input::<u32>("Mod number: ", 20);

    {
        let resp = requester
            .get(url_maker::ModRinth::search_for(properties.limit, 0 * 20))
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
                        .get(url_maker::ModRinth::search_for(
                            properties.limit,
                            properties.page * 20,
                        ))
                        .await?;
                    actual_page = resp.json::<RinthResponse>().await?;
                    pages.insert(properties.page, actual_page.clone());
                } else {
                    actual_page = pages.get(&properties.page).unwrap().clone();
                }
            }

            CODES::ModSelected => {
                let resp = requester
                    .get(url_maker::ModRinth::mod_versions(
                        &actual_page.hits[properties.selected_mod],
                    ))
                    .await?;

                let m_versions = resp.json::<Vec<RinthVersion>>().await?;
                let minecraft_mod = RinthVersions {
                    versions: m_versions,
                };
                println!("{}", minecraft_mod);
                match download_mod(&minecraft_mod, &requester).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Runtime Error => {}", e.to_string());
                    }
                }
                let _ = easy_input::input::<String>("Press enter to continue...", " ".to_string());
            }

            _ => break,
        }
        clear_screen();
    }

    Ok(())
}
