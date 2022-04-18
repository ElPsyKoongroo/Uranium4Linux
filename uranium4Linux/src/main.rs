#![allow(non_snake_case)]
mod checker;
mod code_functions;
mod easy_input;
mod modpack_loader;
mod modpack_maker;
mod variables;

use code_functions::{download_modpack, update};
use modpack_maker::maker::make_modpack;
use std::env;
use variables::constants::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "-d" => return download_modpack(&args[2], &args[3]).await,
        "-u" => update(args[2].as_str()).await,
        "-m" => make_modpack(args[2].as_str()).await,

        "-h" => {
            println!("{}", HELP);
        }
        _ => {
            println!("{}", "Invalid arguments");
        }
    }
    Ok(())
}

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let mut requester = Requester::new();
    let mut pages: HashMap<u32, RinthResponse> = HashMap::new();
    let mut actual_page: RinthResponse;
    let mut properties = Properties::new();

    requester.set_headers(headers.clone());


    let input = easy_input::input("Chose mod repo (RINTH/CURSE): ", String::from("RINTH"));
    let repo = input.as_str();
    println!("Chosen repo: {}", repo);
    load_headers(repo.to_uppercase().as_str(), &mut headers);


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
            CODES::PageSelected =>
                page_selection(
                    &mut pages,
                    &mut properties,
                    &mut requester,
                    &mut actual_page,
                )
                .await,
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
*/
