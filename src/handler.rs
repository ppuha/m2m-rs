use actix_web::web;
use serde::Deserialize;

#[path = "./client.rs"]
mod client;

#[path = "./token.rs"]
mod token;

#[derive(Deserialize)]
pub struct TokenForm {
    client_id: String,
    client_secret: String,
    _grant_type: String,
}

pub async fn get_token(form: web::Form<TokenForm>) -> web::Json<token::Token> {
    let form_data = form.into_inner();
    let client = client::auth_client(form_data.client_id, form_data.client_secret);
    let token = token::generate_token(client.client_id);

    web::Json(token)
}
