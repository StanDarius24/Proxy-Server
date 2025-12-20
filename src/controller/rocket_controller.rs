use crate::model::proxy_server::ProxyServer;
use reqwest::ClientBuilder;
use rocket::async_trait;
use rocket::futures::FutureExt;
use rocket::http::Method;
use rocket::route::{BoxFuture, Outcome};
use rocket::{Data, Request, Route};

use once_cell::sync::Lazy;
use rocket::response::content::RawText;

static STATIC_SERVER: Lazy<ProxyServer> = Lazy::new(|| {
    let yaml_content =
        std::fs::read_to_string("config.yml").expect("Failed to read configuration file");
    serde_yaml::from_str(Box::leak(yaml_content.into_boxed_str()))
        .expect("Failed to parse configuration file")
});

pub async fn dynamic_routes(server: ProxyServer) -> Vec<Route> {
    let dynamic_path = format!("{}/<_..>", STATIC_SERVER.receiver.url);
    vec![Route::new(Method::Get, &dynamic_path, dynamic_test)]
}

#[async_trait]
pub trait AsyncResponder<'r> {
    async fn respond_async(r: &'r Request<'_>, data: Data<'r>) -> Outcome<'r>;
}

pub fn dynamic_test<'r>(r: &'r Request<'_>, data: Data<'r>) -> BoxFuture<'r, Outcome<'r>> {
    async move {
        println!("URI: {}", r.uri());
        if let Some(query) = r.uri().query() {
            println!("Query Parameters: {}", query);
        }
        println!();
        println!("Headers:");
        for header in r.headers().iter() {
            println!("{}: {}", header.name(), header.value());
        }
        let response = fetch_data_example("http://0.0.0.0:80/test1".to_string()).await;
        println!("{}", response);

        Outcome::from(r, RawText(response))
    }
        .boxed()
}

pub async fn fetch_data_example(url: String) -> String {
    let client = ClientBuilder::new()
        .build()
        .expect("Failed to build reqwest client");

    match client.get(url).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => body,
            Err(_) => String::from("Error reading response body"),
        },
        Err(_) => String::from("Failed to make request"),
    }
}
