use reqwest::Response;
use tokio::task::JoinHandle;


pub async fn write_mod(path: &str, res: Response, name: &str){
    let web_res = res;
    let full_path = path.to_owned() + name;
    let content = web_res.bytes().await.unwrap();
    tokio::fs::write(full_path, content).await.unwrap();
}

pub async fn get_writters(responses: Vec<Response>, names: Vec<String>, destination_path: &str) -> Vec<JoinHandle<()>> {
    let mut writters = Vec::new();
    let mut i = 0;
    for response in responses.into_iter(){
        let path_ref = destination_path.to_owned();
        let mod_name = names[i].clone();
                                           
        let task = async move {
            write_mod(
                &path_ref,
                response,
                &mod_name
            ).await;
        };
        writters.push(tokio::spawn(task));
        i += 1;
    }
    writters
}
