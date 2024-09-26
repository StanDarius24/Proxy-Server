use rocket::async_trait;
use rocket::{Data, Request, Route};
use rocket::http::{Method};
use reqwest::ClientBuilder;
use rocket::futures::FutureExt;
use rocket::route::{BoxFuture, Outcome};
use rocket::serde::json::Json;
use crate::model::proxy_server::ProxyServer;

pub async fn dynamic_routes(server: ProxyServer) -> Vec<Route> {
    let dynamic_path = format!("{}/<_..>", server.receiver.url);
    vec![Route::new(Method::Get, &dynamic_path, dynamic_test)]
}

#[async_trait]
pub trait AsyncResponder<'r> {
    async fn respond_async(r: &'r Request<'_>, data: Data<'r>) -> Outcome<'r>;
}

pub fn dynamic_test<'r>(r: &'r Request<'_>, data: Data<'r>) -> BoxFuture<'r, Outcome<'r>> {
    async move {
        let output = String::new();

        println!("URI: {}", r.uri());
        if let Some(query) = r.uri().query() {
            println!("Query Parameters: {}", query);
        }
        println!();
        println!("Headers:");
        for header in r.headers().iter() {
            println!("{}: {}", header.name(), header.value());
        }
        fetch_data_example("http://0.0.0.0:8080/test".to_string()).await;
        println!("{}", output);

        Outcome::from(r, ())
    }.boxed()
}

pub async fn fetch_data_example(url: String) -> Json<String> {
    let client = ClientBuilder::new()
        .build()
        .expect("Failed to build reqwest client");

    match client.get(url).send().await {
        Ok(response) => {

            match response.text().await {
                Ok(body) => Json(body),
                Err(_) => Json(String::from("Error reading response body")),
            }
        }
        Err(_) => Json(String::from("Failed to make request")),
    }
}
