use reqwest::{header::HeaderMap, Response};

use super::load_headers;


pub struct Requester {
    cliente: reqwest::Client,
    headers: Option<HeaderMap>,
}

impl Requester {
    pub fn new() -> Requester {
        let mut headers = HeaderMap::new();
        load_headers::load_headers("RINTH", &mut headers);
        Requester {
            cliente: reqwest::Client::new(),
            headers: Some(headers),
        }
    }

    pub async fn get(&self, url: String) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let resp: Response;
        match self.headers.clone() {
            Some(h) => {
                resp = self.cliente.get(url).headers(h).send().await?;
            }
            None => {
                resp = self.cliente.get(url).send().await?;
            }
        }
        Ok(resp)
        
    }

    #[allow(dead_code)]
    pub async fn get_with_headers(
        &self,
        url: String,
        headers: HeaderMap,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let resp = self.cliente.get(url).headers(headers).send().await?;
        Ok(resp)
    }

    pub fn set_headers(&mut self, headers: HeaderMap) {
        self.headers = Some(headers);
    }
}
