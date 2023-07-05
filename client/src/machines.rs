use std::collections::HashMap;

use serde::{Deserialize, DeserializeOwned, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MachineId(pub String);

#[derive(Serialize)]
pub struct CreateMachine {
    pub name: String,
    pub config: MachineConfig,
}

#[derive(Serialize)]
pub struct MachineConfig {
    pub image: String,
    pub size: String,
    pub env: HashMap<String, String>,
    pub auto_destroy: bool,
}

#[derive(Serialize)]
pub struct CreateMachineResult {
    pub id: MachineId,
}

pub struct Client {
    http: reqwest::Client,
    app_name: String,
}

impl Client {
    pub fn new(auth_token: String, app_name: String) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {auth_token}"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Client { http }
    }

    pub async fn create(&self, args: &CreateMachine) -> anyhow::Result<CreateMachineResult> {
        let app_name = &self.app_name;
        let url = format!("https://api.machines.dev/v1/apps/{app_name}/machines");

        let response = self.post(url)
            .json(args)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("create machine API request returned status: {}", response.status())
        }
    }
}

async fn request<T: DeserializeOwned>(request: reqwest::Request) -> anyhow::Result<T> {
    request.build()
    if !response.status.is_sucess() {
        anyhow::bail!()
    }
}
