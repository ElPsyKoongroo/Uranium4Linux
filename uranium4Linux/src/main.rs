#![allow(unused_imports)]
#![allow(non_snake_case)]
use requester::requester::load_headers::*;
use requester::requester::request_maker::*;
use reqwest::header::HeaderMap;
use serde_json::to_string;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::read_dir;

mod checker;
use crate::checker::check;
use crate::variables::constants::MENU;

use minecraft_mod::minecraft_mod::*;
use minecraft_mod::responses::*;
use modpack_loader::modpack_loader::*;

mod variables;
use crate::variables::*;

mod code_functions;
use crate::code_functions::*;

mod easy_input;
use crate::easy_input::input;

// mod url_maker;
// use crate::url_maker::*;

use minecraft_mod::url_maker::maker;

fn menu(properties: &mut Properties) -> CODES {
    println!("{}", MENU);

    let user_input: Vec<String> = {
        let aux = easy_input::input("Chose an option: ", String::from(" "));
        aux.split(" ").map(|x| x.to_string()).collect()
    };

    match user_input.len() {
        1 => one_input(user_input[0].clone()),
        2 => two_inputs(user_input[0].clone(), user_input[1].as_str(), properties),
        _ => CODES::ParseError,
    }
}

async fn page_selection(
    pages: &mut HashMap<u32, RinthResponse>,
    properties: &mut Properties,
    requester: &mut Requester,
    actual_page: &mut RinthResponse,
) {
    if !pages.contains_key(&properties.get_page()) {
        let resp = requester
            .get(maker::ModRinth::search_for(
                properties.get_limit(),
                properties.get_page() * 20,
            ))
            .await
            .unwrap();
        *actual_page = check(resp.json::<RinthResponse>().await, true).unwrap_or_default();
        if actual_page.len() == 0 {
            println!("This page is empty, nothing here !");
        } else {
            pages.insert(properties.get_page(), actual_page.clone());
        }
    } else {
        *actual_page = pages.get(&properties.get_page()).unwrap().clone();
    }
}

async fn mod_selection(
    properties: &mut Properties,
    requester: &mut Requester,
    actual_page: &mut RinthResponse,
) {
    let actual_mod = &actual_page.hits[properties.get_selected_mod()];
    let resp = requester
        .get(maker::ModRinth::mod_versions(actual_mod))
        .await
        .unwrap();
    println!(
        "\n\n{}\n{}",
        actual_mod.get_title().to_uppercase(),
        actual_mod.get_description()
    );
    let m_versions = check(resp.json::<Vec<RinthVersion>>().await, true).unwrap_or_default();
    let minecraft_mod = RinthVersions {
        versions: m_versions,
    };
    println!("{}", minecraft_mod);
    match download_mod(&minecraft_mod, &requester, &properties.get_path()).await {
        Ok(_) => {}
        Err(e) => println!("Runtime Error => {}", e.to_string()),
    }
    let _ = easy_input::input::<String>("Press enter to continue...", String::from(" "));
}

async fn make_modpack(requester: &mut Requester) {
    let input = easy_input::input("Path: ", String::from("-"));
    let path = Path::new(input.as_str());
    let a = get_mods(path).unwrap();
    let mut responses: Vec<RinthVersion> = Vec::new();
    for item in a {
        let response = {
            let request = requester.get(maker::ModRinth::hash(&item.0)).await.unwrap();
            check(request.json::<RinthVersion>().await, false)
        };
        match response {
            Some(e) => responses.push(e),
            None => {}
        }
    }
    let mp_name = easy_input::input("Modpack name: ", String::from("Modpack.mm"));
    let mp_version = easy_input::input("Modpack version: ", String::from("1.0"));
    let mp_author = easy_input::input("Modpack author: ", String::from("Anonimous"));
    let mp =
        modpack_struct::ModPack::modpack_from_RinthVers(mp_name, mp_version, mp_author, responses);
    mp.write_mod_pack();

    let _ = easy_input::input("Press enter to continue...", 0);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let mut requester = Requester::new();
    requester.set_headers(headers.clone());

    let mut pages: HashMap<u32, RinthResponse> = HashMap::new();
    let mut actual_page: RinthResponse; // = RinthResponse::new();

    let input = easy_input::input("Chose mod repo (RINTH/CURSE): ", String::from("RINTH"));
    let repo = input.as_str();
    println!("Chosen repo: {}", repo);
    load_headers(repo.to_uppercase().as_str(), &mut headers);

    let mut properties = Properties::new();

    {
        let resp = requester
            .get(maker::ModRinth::search_for(properties.get_limit(), 0 * 20))
            .await?;
        actual_page = resp.json::<RinthResponse>().await?;
        pages.insert(properties.get_page(), actual_page.clone());
    }

    loop {
        actual_page.show();
        match menu(&mut properties) {
            CODES::PageSelected => page_selection(&mut pages,&mut properties,&mut requester,&mut actual_page,).await,
            CODES::ModSelected => mod_selection(&mut properties, &mut requester, &mut actual_page).await,
            CODES::SetPath => properties.set_path(set_path()),
            CODES::MakeModPack => make_modpack(&mut requester).await,
            CODES::Exit => break,
            _ => break,
        }
        clear_screen();
    }

    Ok(())
}
