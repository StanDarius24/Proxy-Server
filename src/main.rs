mod configuration;
mod controller;
mod model;

use crate::configuration::server;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    server::Server::create_server(
        Box::from("0.0.0.0"),
        Box::from("8081")
    ).await
}
