use log::info;

pub async fn greet(name: &str) -> String {
    info!("Receive data: {}", name);
    format!("Hello {}!", name)
}
