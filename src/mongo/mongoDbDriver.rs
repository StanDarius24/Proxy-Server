use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection,
};
use mongodb::error::Result;

#[derive(Clone)]
pub struct MongoService {
    proxy_entries: Collection<Document>,
}

impl MongoService {
    pub async fn new() -> Result<Self> {
        let uri = "mongodb://root:password@localhost:27017/?authSource=admin";

        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;

        let db = client.database("proxy_server_db");
        let proxy_entries = db.collection::<Document>("entities");

        Ok(Self { proxy_entries })
    }

    pub async fn insert_proxy_entry(&self, proxy_entries: Document) -> Result<mongodb::bson::oid::ObjectId> {
        let result = self.proxy_entries.insert_one(proxy_entries).await?;
        Ok(result
            .inserted_id
            .as_object_id()
            .expect("Should have inserted ObjectId"))
    }

    pub async fn get_entry_by_id(&self, id: &mongodb::bson::oid::ObjectId) -> Result<Option<Document>> {
        let filter = doc! { "_id": id };
        let user = self.proxy_entries.find_one(filter).await?;
        Ok(user)
    }

    pub async fn get_entry_by_key(&self, key: &str) -> Result<Option<Document>> {
        let filter = doc! { "key": key };
        let proxy_entries = self.proxy_entries.find_one(filter).await?;
        Ok(proxy_entries)
    }
}