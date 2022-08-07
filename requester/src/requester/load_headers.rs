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
            headers.insert(
                AUTHORIZATION,
                "$2a$10$6mO.gbGdm7elhecL3XMcxOby5RrftY2ufGTZxg3gocM1kDlF1UCuK"
                    .parse()
                    .unwrap(),
            );
        }
        _ => {
            println!("Wrong API name, using RINTH");
        }
    }
}
