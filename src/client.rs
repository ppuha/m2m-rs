use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Client {
    pub client_id: String,
    pub client_secret: Option<String>,
}

pub trait ClientStore {
    async fn get_all(&self) -> Result<Vec<Client>, String>;
    async fn get(&self, client_id: String) -> Result<Client, String>;
}

pub async fn auth_client(
    client_id: String,
    client_secret: String,
    store: &impl ClientStore,
) -> Result<Client, String> {
    store.get(client_id).await.and_then(|client| {
        if client_secret_match(&client, client_secret) {
            Ok(client)
        } else {
            Err("invalid client credentials".to_string())
        }
    })
}

fn client_secret_match(client: &Client, client_secret: String) -> bool {
    client
        .client_secret
        .to_owned()
        .map(|cs| cs == client_secret)
        .or_else(|| Some(true))
        .unwrap()
}
