use rocket::{figment::Figment, Config};

use crate::controller::rocket_controller;

pub struct Server {}

impl Server {

    pub async fn new(host: Box<str>, port: Box<str>) -> Result<(), rocket::Error> {
        let figment: Figment = Server::config(host, port);
        let routes: Vec<rocket::Route> = Server::routes();

        let _rocket: rocket::Rocket<rocket::Ignite> = rocket::custom(figment)
        .mount("/", routes)
        .launch()
        .await?;

        Ok(())
    }

    fn config(host: Box<str>, port: Box<str>) -> Figment {
        let port: i32 = port.parse().expect("The provided port is not a valid integer.");

        let figment: Figment = Config::figment()
        .merge(("address", host.to_string()))
        .merge(("port", port));

        figment
    }

    fn routes() -> Vec<rocket::Route> {
        let mut routes: Vec<rocket::Route> = vec![];
    
        routes.extend(rocket_controller::routes());
    
        routes
    }
}

