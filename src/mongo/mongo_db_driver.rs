use mongodb::error::Result;
use mongodb::{bson::{doc, DateTime, Document}, options::ClientOptions, Client, Collection, IndexModel};
use mongodb::options::IndexOptions;
use tokio::sync::OnceCell;

static MONGO_SERVICE: OnceCell<MongoService> = OnceCell::const_new();

pub async fn get_mongo_service() -> &'static MongoService {
    MONGO_SERVICE
        .get_or_init(|| async { MongoService::new().await.expect("Failed to create service") })
        .await
}

#[derive(Clone)]
pub struct MongoService {
    proxy_entries: Collection<Document>,
}

impl MongoService {
    pub async fn new() -> Result<Self> {
        let uri = "mongodb://root:password@localhost:27017/?authSource=admin";

        let index_options = IndexOptions::builder().expire_after(Some(std::time::Duration::from_secs(3600))).build();
        let index_model = IndexModel::builder()
            .keys(doc! { "created_at": 1 })
            .options(index_options)
            .build();

        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;

        let db = client.database("proxy_server_db");
        let proxy_entries = db.collection::<Document>("entities");

        proxy_entries.create_index(index_model).await?;

        Ok(Self { proxy_entries })
    }

    pub async fn insert_proxy_entry(
        &self,
        mut proxy_entries: Document,
    ) -> Result<mongodb::bson::oid::ObjectId> {
        proxy_entries.insert("created_at", DateTime::now());
        let result = self.proxy_entries.insert_one(proxy_entries).await?;
        Ok(result
            .inserted_id
            .as_object_id()
            .expect("Should have inserted ObjectId"))
    }

    pub async fn get_entry_by_id(
        &self,
        id: &mongodb::bson::oid::ObjectId,
    ) -> Result<Option<Document>> {
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
