use reqwest::header::{HeaderMap, AUTHORIZATION};
mod minecraft_mod;
mod url_maker;
mod request_maker;
mod responses;

const RINTH: &str = "Rinth";
const CURSE: &str = "Curse";

fn load_headers(api: &str, headers: &mut HeaderMap){
    match api {
        RINTH => {
            headers.insert(AUTHORIZATION, "Bearer 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".parse().unwrap());
        }
        CURSE => {
            headers.insert("Accept", "application/json".parse().unwrap());
            headers.insert("x-api-key", "$2a$10$VlTkbyk.57PemPJAOTPEVel1mqbpgcZ8H2hkMuTp21Q50DSKvROku".parse().unwrap());
        }
        _ => {
            println!("Wrong API name, nothing loaded");
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    let requester = request_maker::Requester::new();    
    load_headers(RINTH, &mut headers);
    
    let resp = requester.get_with_headers(
        url_maker::ModRinth::search_for(20, 0), 
        headers.clone()
    ).await?; 


    println!("Sending the request...");
    let response_json = resp.json::<responses::RinthResponse>().await?;
    println!("{:#?}", response_json);
    
    /*for minecraft_mod in response_json.hits.iter_mut(){
        minecraft_mod.sort();
    }

    response_json.to_json()?; 
    
    println!("{:#?}", response_json);*/


    Ok(())
}