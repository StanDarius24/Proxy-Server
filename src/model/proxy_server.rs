use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProxyServer {
    pub receiver: ServerDetails,
    pub sender: ServerDetails,
}

#[derive(Debug, Deserialize)]
pub struct ServerDetails {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub authorization: HashMap<String, String>,
    pub body: String
}
