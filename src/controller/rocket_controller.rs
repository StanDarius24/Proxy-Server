use crate::model::proxy_server::{EntitiesConfig, RequestDetails};
use reqwest::ClientBuilder;
use rocket::async_trait;
use rocket::futures::FutureExt;
use rocket::http::Method;
use rocket::route::{BoxFuture, Outcome};
use rocket::{Data, Request, Route};

use crate::memory::l2_cache::L2Cache;
use once_cell::sync::Lazy;
use rocket::response::content::RawText;
use crate::memory::l2_cache::L2Cache;

static STATIC_SERVER: Lazy<EntitiesConfig> = Lazy::new(|| {
    let yaml_content =
        std::fs::read_to_string("config.yml").expect("Failed to read configuration file");
    serde_yaml::from_str(Box::leak(yaml_content.into_boxed_str()))
        .expect("Failed to parse configuration file")
});

pub async fn dynamic_routes(_server: EntitiesConfig) -> Vec<Route> {
    let mut routes: Vec<Route> = Vec::new();
    for entity in &STATIC_SERVER.entities {
        let method = match entity.incoming_request.method.to_uppercase().as_str() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "DELETE" => Method::Delete,
            _ => Method::Get,
        };
        let path = format!("{}/<_..>", entity.incoming_request.endpoint_path);
        routes.push(Route::new(method, &path, dynamic_test));
    }
    routes
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

        let outgoing_request = verify_configuration_sender_receiver(r);

        let (entity_name, request_details) =
            (outgoing_request.0.clone(), outgoing_request.1.clone());

        let response = fetch_data_example(request_details.clone()).await;
        println!("{}", response);

        let entity_name_clone = entity_name.clone();
        let request_details_clone = request_details.clone();
        let response_clone = response.clone();

        tokio::task::spawn_blocking(move || {
            tokio::runtime::Handle::current().block_on(
                L2Cache::store_data(entity_name_clone, request_details_clone, response_clone)
            );
        });

        Outcome::from(r, RawText(response))
    }
        .boxed()
}

fn verify_configuration_sender_receiver(r: &Request<'_>) -> (String, RequestDetails) {
    let req_host = r.headers().get_one("Host").unwrap_or_default();
    let req_method = r.method().as_str();
    let req_path = r.uri().path().to_string();

    let matched = STATIC_SERVER.entities.iter().find(|entity| {
        let incoming = &entity.incoming_request;
        let url_matches = req_path == incoming.endpoint_path;
        let method_matches = req_method.eq_ignore_ascii_case(&incoming.method);
        let host_matches = req_host == incoming.server_host;
        url_matches && method_matches && host_matches
    });

    if let Some(entity) = matched {
        (entity.name.clone(), entity.outgoing_request.clone())
    } else {
        panic!("Request does not match any incoming_request configuration");
    }
}

pub async fn fetch_data_example(request: RequestDetails) -> String {
    let client = ClientBuilder::new()
        .build()
        .expect("Failed to build reqwest client");

    let url = format!("http://{}{}", request.server_host, request.endpoint_path);
    let mut req = match request.method.to_uppercase().as_str() {
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "GET" | _ => client.get(&url),
    };

    for (key, value) in &request.headers {
        req = req.header(key, value);
    }

    if !request.query_params.is_empty() {
        req = req.query(&request.query_params);
    }

    if matches!(request.method.to_uppercase().as_str(), "POST" | "PUT")
        && !request.payload.is_empty()
    {
        req = req.body(request.payload.clone());
    }

    match req.send().await {
        Ok(response) => response
            .text()
            .await
            .unwrap_or_else(|_| String::from("Error reading response body")),
        Err(_) => String::from("Failed to make request"),
    }
}
