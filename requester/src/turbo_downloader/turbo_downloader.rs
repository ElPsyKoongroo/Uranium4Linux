async fn download_v2(
    &mut self,
    mut not_done_mods: Vec<usize>,
    mut responses: Vec<JoinHandle<Response>>,
    minecraft_mods: Vec<Mods>,
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    let start = Instant::now();
    loop {
        let done_mod = self.download_loop(not_done_mods.clone(), &mut responses, &minecraft_mods).await;
        not_done_mods.retain(|&x| x != done_mod);
    
        if not_done_mods.is_empty() {
            break;
        }
    }
    #[cfg(debug_assertions)]
    print!("{:<3}\n", start.elapsed().as_millis());
    
    Ok(())
}


async fn download_loop(
    &mut self,
    not_done_mods: Vec<usize>,
    responses: &mut Vec<JoinHandle<Response>>,
    minecraft_mods: &Vec<Mods>,
) -> usize{
    for i in not_done_mods.clone() {
        let sleep = time::sleep(Duration::from_millis(50));
        tokio::pin!(sleep);


        tokio::select! {
            _ = &mut sleep =>  {
                continue;
            }
            
            res = &mut responses[i] => {
                let web_res = res.unwrap();
                let full_path = self.path.clone() + minecraft_mods.index(i).get_file_name().as_ref(); 
                let content = web_res.bytes().await.unwrap();
                tokio::fs::write(full_path, content).await.unwrap();
            }
            
            else => {
                break;
            }
        }
        return i;
    }
    0
}