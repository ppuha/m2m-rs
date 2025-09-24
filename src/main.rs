use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use env_logger::{Env, init_from_env};

mod handler;

fn init_logger() {
    init_from_env(Env::new().default_filter_or("info"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    HttpServer::new(|| {
        App::new()
            .route("oauth2/token", web::post().to(handler::get_token))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 5500))?
    .run()
    .await
}
