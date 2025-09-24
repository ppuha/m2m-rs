pub struct Client {
    pub client_id: String,
    client_secret: String,
}

pub fn auth_client(client_id: String, client_secret: String) -> Client {
    Client {
        client_id: client_id,
        client_secret: client_secret,
    }
}
