mod configuration;
mod controller;

use crate::configuration::server;

#[async_std::main]
async fn main() -> tide::Result<()> {
 server::Server::new(Box::from("0.0.0.0"), Box::from("8080")).await
}


