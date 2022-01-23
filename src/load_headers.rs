use reqwest::header::{HeaderMap, AUTHORIZATION};
const RINTH: &str = "Rinth";
const CURSE: &str = "Curse";
pub fn load_headers(api: &str, headers: &mut HeaderMap){
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