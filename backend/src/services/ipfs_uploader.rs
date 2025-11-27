use reqwest::{
    Client,
    multipart::{Form, Part},
};

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
        let file_part = Part::bytes(chunk.as_bytes().to_vec())
            .file_name("data.txt")
            .mime_str("application/octet-stream")?;

        let form = Form::new().part("file", file_part);
        
        let response = self.client.post(&self.endpoint).header("Authorization", "Bearer NDhERDFCRDM2QkIzMjU4RDA1MzY6TFA5UnFRSzB5Y3lxUG1tOG13ZHJZN1A2Z2ZaeGhBMnY4aVdwRHRqTDp0ZXN0YnVja2Vy").multipart(form).send().await?;

        Ok(response.text().await?)
    }
}
