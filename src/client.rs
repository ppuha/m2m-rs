#[derive(Clone)]
pub struct Client {
    pub client_id: String,
    pub client_secret: Option<String>,
}

pub trait ClientStore {
    async fn get(&self, client_id: String) -> Option<Client>;
}

pub async fn auth_client(
    client_id: String,
    client_secret: String,
    store: &impl ClientStore,
) -> Result<Client, String> {
    let client = store.get(client_id).await;

    match client {
        Some(c) => {
            if c.client_secret
                .to_owned()
                .map(|cs| cs == client_secret)
                .or_else(|| Some(true))
                .unwrap()
            {
                Ok(c.clone())
            } else {
                Err("invalid credentials".to_string())
            }
        }
        None => Err("unknown client".to_string()),
    }
}
