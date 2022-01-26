#[allow(unused_imports)]
use crate::load_headers::load_headers;
use crate::easy_input::input;

use reqwest::header::{HeaderMap};
use serde_json::to_string;
use variables::{RINTH, CURSE};

mod variables;
mod easy_input;
mod load_headers;
mod minecraft_mod;
mod url_maker;
mod request_maker;
mod responses;

struct Properties{
    limit: u32,
    page: u32,
    offset: u32
}


fn menu(properties: &mut Properties){

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

    match option{

        "mod" => {
            println!("value: {}", value);
        }

        "page" => {
            properties.page = value.parse::<u32>().expect(" F ");
        }

        _ => {
            println!("Ooops!");
        }
    }

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let requester = request_maker::Requester::new();  
    
    


    let input = easy_input::input_string("Chose mod repo (RINTH/CURSE): ", "RINTH".to_string());
    let repo = input.as_str();
    println!("Chosen repo: {}", repo);
    load_headers(repo.to_uppercase().as_str(), &mut headers);
    
    let mut properties = Properties{limit: 0, page: 0, offset: 0};
    properties.limit = easy_input::input::<u32>("Mod number: ", 20);

    loop {
        menu(&mut properties);

        let resp = requester.get_with_headers(
            url_maker::ModRinth::search_for(properties.limit, properties.page*20), 
            headers.clone()
        ).await?; 
        
        let actual_page = resp.json::<responses::RinthResponse>().await?;

        for (index, minecraft_mod) in  actual_page.hits.iter().enumerate(){
            println!("{:2}: {}", index, minecraft_mod.to_string());
        }

        let _ = easy_input::input::<String>("Press enter to continue...", " ".to_string());
        print!("\x1B[2J\x1B[1;1H");

        if false == true{break;}
    }
    
    Ok(())
}
