use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProxyServer {
    receiver: ServerDetails,
    sender: ServerDetails,
}

#[derive(Debug, Deserialize)]
pub struct ServerDetails {
    url: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    authorization: HashMap<String, String>,
    body: String
}
