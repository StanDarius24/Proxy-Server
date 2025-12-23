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
        let base_path = entity.incoming_request.endpoint_path.trim_end_matches("/*");
        let path = format!("{}/<_..>", base_path);
        routes.push(Route::new(method, &path, dynamic_test));
    }
    routes
}

#[async_trait]
pub trait AsyncResponder<'r> {
    async fn respond_async(r: &'r Request<'_>, data: Data<'r>) -> Outcome<'r>;
}

pub fn dynamic_test<'r>(r: &'r Request<'_>, _data: Data<'r>) -> BoxFuture<'r, Outcome<'r>> {
    async move {
        // Log request URI and query parameters
        println!("URI: {}", r.uri());
        if let Some(query) = r.uri().query() {
            println!("Query Parameters: {}", query);
        }
        println!("\nHeaders:");
        for header in r.headers().iter() {
            println!("{}: {}", header.name(), header.value());
        }

        let (entity_name, request_details) = {
            let outgoing_request = verify_configuration_sender_receiver(r);
            (outgoing_request.0, outgoing_request.1)
        };

        if let Some(doc) = L2Cache::fetch_data(request_details.clone()).await {
            if let Ok(incoming_req) = doc.get_document("incoming_request") {
                if let Ok(body) = incoming_req.get_str("body") {
                    return Outcome::from(r, RawText(body.to_string()));
                }
            }
        }

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
        let endpoint = incoming.endpoint_path.trim_end_matches("/*");
        let url_matches = if incoming.endpoint_path.ends_with("/*") {
            req_path.starts_with(endpoint)
        } else {
            req_path == incoming.endpoint_path
        };
        let method_matches = req_method.eq_ignore_ascii_case(&incoming.method);
        let host_matches = req_host == incoming.server_host;
        url_matches && method_matches && host_matches
    });

    if let Some(entity) = matched {
        let mut outgoing_request = entity.outgoing_request.clone();
        let incoming = &entity.incoming_request;
        if incoming.endpoint_path.ends_with("/*") {
            let base = incoming.endpoint_path.trim_end_matches("/*");
            if let Some(suffix) = req_path.strip_prefix(base) {
                // Ensure outgoing path ends with / if needed
                let outgoing_base = outgoing_request.endpoint_path.trim_end_matches("/*").to_string();
                outgoing_request.endpoint_path = format!("{}{}", outgoing_base, suffix);
            }
        }
        (entity.name.clone(), outgoing_request)
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
