use ripro_manager::{
    server::start_server,
    config::AppConfig,
};

#[actix_web::main]
async fn main() -> Result<(), Box<std::io::Error>> {
    dotenv::dotenv().ok();
    let config = AppConfig::from_env();
    start_server(config).await?;
    Ok(())
}
