use crate::model::proxy_server::{Entity, RequestDetails};
use crate::mongo::mongo_db_driver::get_mongo_service;
use mongodb::bson::Document;

pub struct L2Cache {}

impl L2Cache {
    pub async fn store_data(name: String, request_details: RequestDetails, body: String) {
        let service = get_mongo_service().await;

        let incoming = &request_details;
        let composite_key = format!(
            "{}|{}|{}",
            incoming.method, incoming.server_host, incoming.endpoint_path
        );

        let mut doc = Document::new();
        doc.insert("name", name);
        doc.insert("key", composite_key);

        let mut incoming_doc = Document::new();
        incoming_doc.insert("method", incoming.method.clone());
        incoming_doc.insert("server_host", incoming.server_host.clone());
        incoming_doc.insert("endpoint_path", incoming.endpoint_path.clone());
        incoming_doc.insert("body", body);

        doc.insert("incoming_request", incoming_doc);

        service.insert_proxy_entry(doc).await.unwrap();
    }

    pub async fn fetch_data(request_details: RequestDetails) -> Option<Document> {
        let service = get_mongo_service().await;

        let incoming = &request_details;
        let composite_key = format!(
            "{}|{}|{}",
            incoming.method, incoming.server_host, incoming.endpoint_path
        );

        match service.get_entry_by_key(&composite_key).await {
            Ok(Some(response)) => Some(response),
            _ => None,
        }
    }
}
