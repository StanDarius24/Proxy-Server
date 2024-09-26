use rocket::{figment::Figment, Config};
use crate::configuration::parser::Parser;
use crate::controller::rocket_controller;

pub struct Server {}

impl Server {

    pub async fn create_server(host: Box<str>, port: Box<str>) -> Result<(), rocket::Error> {
        let figment: Figment = Server::configure_port_address(host, port);
        let routes: Vec<rocket::Route> = Server::routes().await;

        let _rocket: rocket::Rocket<rocket::Ignite> = rocket::custom(figment)
        .mount("/", routes)
        .launch()
        .await?;

        Ok(())
    }

    fn configure_port_address(host: Box<str>, port: Box<str>) -> Figment {
        let port: i32 = port.parse().expect("The provided port is not a valid integer.");

        let figment: Figment = Config::figment()
        .merge(("address", host.to_string()))
        .merge(("port", port));

        figment
    }

    async fn routes() -> Vec<rocket::Route> {
        let mut routes: Vec<rocket::Route> = vec![];

        // routes.extend(rocket_controller::routes());
        routes.extend(rocket_controller::dynamic_routes(Parser::read_configuration()).await);


        routes
    }
}

