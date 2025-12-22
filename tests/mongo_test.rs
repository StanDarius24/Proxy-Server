#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::{doc, Document, oid::ObjectId};
    use tokio;
    use server_proxy::mongo::mongo_db_driver::MongoService;

    #[tokio::test]
    async fn test_insert_and_get_user() {
        let service = MongoService::new().await.expect("Failed to create service");

        let mut user = Document::new();
        user.insert("name", "test_user");
        user.insert("email", "test@example.com");

        let inserted_id = service.insert_proxy_entry(user.clone()).await.expect("Insert failed");
        assert!(inserted_id.to_string().len() > 0);

        let fetched = service.get_entry_by_id(&inserted_id).await.expect("Get failed");
        assert!(fetched.is_some());
        let fetched_doc = fetched.unwrap();
        assert_eq!(fetched_doc.get_str("name").unwrap(), "test_user");
        assert_eq!(fetched_doc.get_str("email").unwrap(), "test@example.com");
    }
}
