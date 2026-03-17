use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::client::{ClientStore, auth_client};
use crate::static_store::StaticStore;
use crate::token::generate_token;

pub async fn get_clients(store: web::Data<StaticStore>) -> HttpResponse {
    let res = store.get_ref().get_all().await;

    match res {
        Ok(clients) => HttpResponse::Ok().json(Json(clients)),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

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

pub async fn get_token(store: web::Data<StaticStore>, form: web::Form<TokenForm>) -> HttpResponse {
    let form_data = form.into_inner();
    if form_data.grant_type != "client_credentials" {
        HttpResponse::BadRequest().json(Json(TokenError {
            error: "invalid grant type".to_string(),
        }))
    } else {
        let auth_result = auth_client(
            form_data.client_id,
            form_data.client_secret,
            store.get_ref(),
        )
        .await;
        match auth_result {
            Ok(client) => {
                let token = generate_token(client.client_id);
                HttpResponse::Ok().json(Json(token))
            }
            Err(err) => HttpResponse::BadRequest().json(TokenError { error: err }),
        }
    }
}
