mod configuration;
mod controller;
mod model;

use server_proxy::configuration::parser::Parser;
use crate::configuration::server;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    Parser::read_configuration();
    server::Server::new(Box::from("0.0.0.0"), Box::from("8080")).await
}