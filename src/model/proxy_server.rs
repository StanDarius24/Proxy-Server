use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct EntitiesConfig {
    pub entities: Vec<Entity>,
}

#[derive(Debug, Deserialize)]
pub struct Entity {
    pub name: String,
    pub incoming_request: RequestDetails,
    pub outgoing_request: RequestDetails,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RequestDetails {
    pub method: String,
    pub server_host: String,
    pub endpoint_path: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub auth: HashMap<String, String>,
    pub payload: String,
}