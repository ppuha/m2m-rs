mod client;
mod handler;
mod static_store;
mod token;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use env_logger::{Env, init_from_env};

use crate::static_store::StaticStore;

fn init_logger() {
    init_from_env(Env::new().default_filter_or("info"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let store = web::Data::new(StaticStore::new());

    init_logger();
    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .route("admin/clients", web::get().to(handler::get_clients))
            .route("oauth2/token", web::post().to(handler::get_token))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 5500))?
    .run()
    .await
}
