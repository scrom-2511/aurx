use reqwest::Client;

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

    pub async fn upload_chunk(&self, chunk: String) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client.post(&self.endpoint).body(chunk).send().await?;
        Ok(response.text().await?)
    }
}
