use actix_web::{
    App,
    get,
    HttpResponse,
    HttpServer,
    Responder,
};

use crate::config::AppConfig;

pub async fn start_server(config: AppConfig) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .service(hello)
    }).bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await;

    Ok(())
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Wow!")
}

