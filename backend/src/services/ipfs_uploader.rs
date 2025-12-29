use std::env;

use reqwest::{Client, multipart::Form};

#[derive(Clone)]
pub struct IPFSUploader {
    client: Client,
    endpoint: String,
}

impl IPFSUploader {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.to_string(),
        }
    }

    pub async fn upload(
        &self,
        file_name: &str,
        path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let form = Form::new()
            .file(file_name.to_string(), path.to_string())
            .await?;
        let ipfs_secret = env::var("IPFS_SECRET").unwrap();
        let response = self.client.post(&self.endpoint).header("Authorization", ipfs_secret).multipart(form).send().await?;

        Ok(response.text().await?)
    }
}
