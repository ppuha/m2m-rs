use crate::client::{Client, ClientStore};
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;

pub struct FileStore {
    path: String,
}

impl FileStore {
    pub fn new(path: String) -> Self {
        Self { path: path }
    }
}

impl ClientStore for FileStore {
    async fn get_all(&self) -> Result<Vec<Client>, String> {
        let file = File::open(&self.path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        from_reader::<BufReader<File>, Vec<Client>>(reader).map_err(|e| e.to_string())
    }

    async fn get(&self, client_id: String) -> Result<Client, String> {
        self.get_all()
            .await
            .map(|clients| clients.into_iter().find(|c| c.client_id == client_id))
            .and_then(|co| match co {
                Some(c) => Ok(c),
                None => Err("Client not found".to_string()),
            })
    }
}
