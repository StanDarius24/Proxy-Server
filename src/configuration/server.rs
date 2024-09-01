use crate::controller::crud_controller;

pub struct Server {}

impl Server {
    pub async fn new(host: Box<str>, port: Box<str>) -> tide::Result<()> {

        let app = crud_controller::set_urls().await;

        app.listen(host.to_string() + ":" + &*port).await?;

        Ok(())

    }

}

