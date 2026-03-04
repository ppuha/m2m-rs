#[derive(Clone)]
pub struct Client {
    pub client_id: String,
    client_secret: Option<String>,
}

async fn get_clients() -> Vec<Client> {
    vec![
        Client {
            client_id: "client0".to_string(),
            client_secret: None,
        },
        Client {
            client_id: "client1".to_string(),
            client_secret: Some("foobar".to_string()),
        },
        Client {
            client_id: "client2".to_string(),
            client_secret: Some("secret".to_string()),
        },
    ]
}

pub async fn auth_client(client_id: String, client_secret: String) -> Result<Client, String> {
    let client = get_clients()
        .await
        .into_iter()
        .find(|c| c.client_id == client_id);

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
