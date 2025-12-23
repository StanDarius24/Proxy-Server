use crate::model::proxy_server::RequestDetails;
use crate::valkey::valkey_driver::get_valkey_service;

pub struct L1Cache {}

impl L1Cache {
    pub async fn store_data(
        name: String,
        request_details: RequestDetails,
        body: String,
    ) -> anyhow::Result<()> {
        let service = get_valkey_service().await;

        let composite_key = format!(
            "{}|{}|{}",
            request_details.method, request_details.server_host, request_details.endpoint_path
        );
        let redis_key = format!("{}:{}", name, composite_key);

        eprintln!("Writing key: {}", redis_key);

        service.set_type(&redis_key, &body).await?;

        Ok(())
    }

    pub async fn fetch_data(
        name: String,
        request_details: RequestDetails,
    ) -> anyhow::Result<Option<String>> {
        let service = get_valkey_service().await;

        let composite_key = format!(
            "{}|{}|{}",
            request_details.method, request_details.server_host, request_details.endpoint_path
        );
        let redis_key = format!("{}:{}", name, composite_key);

        eprintln!("Fetching key: {}", redis_key);

        let result = service.get_key(&redis_key).await?;
        Ok(result)
    }
}
