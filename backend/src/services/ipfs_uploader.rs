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

    pub async fn upload(
        &self,
        file_name: &str,
        path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let form = Form::new().file(file_name.to_string(), path.to_string()).await?;

        let response = self.client.post(&self.endpoint).header("Authorization", "Bearer NDhERDFCRDM2QkIzMjU4RDA1MzY6TFA5UnFRSzB5Y3lxUG1tOG13ZHJZN1A2Z2ZaeGhBMnY4aVdwRHRqTDp0ZXN0YnVja2Vy").multipart(form).send().await?;

        Ok(response.text().await?)
    }
}
