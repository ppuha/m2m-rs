use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

#[path = "./client.rs"]
mod client;

#[path = "./token.rs"]
mod token;

#[derive(Deserialize)]
pub struct TokenForm {
    client_id: String,
    client_secret: String,
    grant_type: String,
}

#[derive(Serialize)]
pub struct TokenError {
    error: String,
}

pub async fn get_token(form: web::Form<TokenForm>) -> HttpResponse {
    let form_data = form.into_inner();
    if form_data.grant_type != "client_credentials" {
        HttpResponse::BadRequest().json(Json(TokenError {
            error: "invalid grant type".to_string(),
        }))
    } else {
        let auth_result = client::auth_client(form_data.client_id, form_data.client_secret).await;
        match auth_result {
            Ok(client) => {
                let token = token::generate_token(client.client_id);
                HttpResponse::Ok().json(Json(token))
            }
            Err(err) => HttpResponse::BadRequest().json(TokenError { error: err }),
        }
    }
}
