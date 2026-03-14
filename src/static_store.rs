use crate::client::{Client, ClientStore};

pub struct StaticStore {
    clients: Vec<Client>,
}

impl StaticStore {
    pub fn new() -> Self {
        Self {
            clients: vec![
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
            ],
        }
    }
}

impl ClientStore for StaticStore {
    async fn get(&self, client_id: String) -> Option<Client> {
        self.clients
            .to_owned()
            .into_iter()
            .find(|c| c.client_id == client_id)
    }
}
