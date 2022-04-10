// use crate::variables::{RINTH, CURSE};

// use crate::variables::{constants::*};
pub const RINTH: &str = "RINTH";
pub const CURSE: &str = "CURSE";

use reqwest::header::{HeaderMap, AUTHORIZATION};

pub fn load_headers(api: &str, headers: &mut HeaderMap) {
    match api {
        RINTH => {
            headers.insert(
                AUTHORIZATION,
                "Bearer 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"
                    .parse()
                    .unwrap(),
            );
        }
        CURSE => {
            println!("CURSE repository is not implemente yet, RINTH will be loaded instead!");
            headers.insert(
                AUTHORIZATION,
                "Bearer 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"
                    .parse()
                    .unwrap(),
            );
            /*
            headers.insert("Accept", "application/json".parse().unwrap());
            headers.insert("x-api-key", "$2a$10$VlTkbyk.57PemPJAOTPEVel1mqbpgcZ8H2hkMuTp21Q50DSKvROku".parse().unwrap());
            */
        }
        _ => {
            println!("Wrong API name, using RINTH");
        }
    }
}
