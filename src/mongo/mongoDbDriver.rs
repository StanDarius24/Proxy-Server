use anyhow::Result;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection,
};

#[derive(Clone)]
pub struct MongoService {
    users: Collection<Document>,
}

impl MongoService {
    pub async fn new() -> Result<Self> {
        let uri = "mongodb://root:example@localhost:27017/?authSource=admin";

        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;

        let db = client.database("test_db");
        let users = db.collection::<Document>("users");

        Ok(Self { users })
    }

    pub async fn insert_user(&self, name: &str, email: &str, age: i32) -> Result<String> {
        let doc = doc! {
            "name": name,
            "email": email,
            "age": age
        };

        let result = self.users.insert_one(doc, None).await?;
        Ok(result.inserted_id.to_string())
    }

    pub async fn get_users(&self) -> Result<Vec<Document>> {
        let mut cursor = self.users.find(None, None).await?;
        let mut users = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            users.push(doc);
        }

        Ok(users)
    }
}
