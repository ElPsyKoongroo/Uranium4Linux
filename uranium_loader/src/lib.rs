
pub mod modpack_loader;
pub mod functions;

#[tokio::test]
async fn my_test() {
    use crate::functions::update;
    update("/home/sergio/Documents/programacion/Rust/Uranium4Linux/Sergio2.json".to_string()).await;
    assert!(true);
}