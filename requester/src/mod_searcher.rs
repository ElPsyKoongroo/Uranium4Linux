use serde;
use tokio::{task, task::JoinHandle};

use mine_data_strutcs::url_maker::maker::ModRinth;

use crate::requester::request_maker::{CurseRequester, Req};

pub enum Method {
    GET,
    POST,
}

pub struct RequestInfo {
    pub url: String,
    pub method: Method,
    pub body: String,
}

pub fn search_with_client(
    requester: &CurseRequester,
    request: &RequestInfo,
) -> JoinHandle<Result<reqwest::Response, reqwest::Error>> {
    match request.method {
        Method::GET => requester.get(&request.url, Method::GET, ""),
        Method::POST => requester.get(&request.url, Method::POST, &request.body),
    }
}

pub fn search_mod_by_id(id: &str) -> task::JoinHandle<reqwest::Response> {
    let url = ModRinth::mod_versions_by_id(id);
    let a_func = async {
        let cliente = reqwest::Client::new();
        cliente
            .get(url)
            .header(
                reqwest::header::USER_AGENT,
                "github.com/ElPsyKoongroo/Uranium4Linux (sergious234@gmail.com)"
                    .parse::<reqwest::header::HeaderValue>()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap()
    };
    task::spawn(a_func)
}

pub fn search_version_by_id(id: &str) -> task::JoinHandle<reqwest::Response> {
    let url = ModRinth::mod_version_by_id(id);
    let a_func = async {
        let cliente = reqwest::Client::new();
        cliente
            .get(url)
            .header(
                reqwest::header::USER_AGENT,
                "github.com/ElPsyKoongroo/Uranium4Linux (sergious234@gmail.com)"
                    .parse::<reqwest::header::HeaderValue>()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap()
    };
    task::spawn(a_func)
}

pub fn search_by_url(
    cliente: &reqwest::Client,
    url: &str,
) -> task::JoinHandle<Result<reqwest::Response, reqwest::Error>> {
    let url = url.to_owned();
    tokio::task::spawn(
        cliente
            .get(url)
            .header(
                reqwest::header::USER_AGENT,
                "github.com/ElPsyKoongroo/Uranium4Linux (sergious234@gmail.com)"
                    .parse::<reqwest::header::HeaderValue>()
                    .unwrap(),
            )
            .send(),
    )
}

pub fn search_by_url_owned(
    cliente: reqwest::Client,
    url: &str,
) -> task::JoinHandle<Result<reqwest::Response, reqwest::Error>> {
    let url = url.to_owned();
    tokio::task::spawn(async move {
        cliente
            .get(&url)
            .header(
                reqwest::header::USER_AGENT,
                "github.com/ElPsyKoongroo/Uranium4Linux (sergious234@gmail.com)"
                    .parse::<reqwest::header::HeaderValue>()
                    .unwrap(),
            )
            .send()
            .await
    })
}

pub fn search_by_url_post<T>(
    cliente: &reqwest::Client,
    url: &str,
    content: &T,
) -> task::JoinHandle<Result<reqwest::Response, reqwest::Error>>
where
    T: serde::Serialize,
{
    tokio::task::spawn(
        cliente
            .post(url.to_owned())
            .json(content)
            .header(
                reqwest::header::USER_AGENT,
                "github.com/ElPsyKoongroo/Uranium4Linux (sergious234@gmail.com)"
                    .parse::<reqwest::header::HeaderValue>()
                    .unwrap(),
            )
            .send(),
    )
}
