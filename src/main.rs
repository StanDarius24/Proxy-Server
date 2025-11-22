#[macro_use]
extern crate rocket;
mod configuration;
mod controller;
mod model;

use crate::configuration::server;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    server::Server::create_server(Box::from("0.0.0.0"), Box::from("8080")).await
}
