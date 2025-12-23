use redis::Client;
use tokio::sync::OnceCell;

pub struct ValkeyService {
    pub client: Client,
}
static VALKEY_SERVICE: OnceCell<ValkeyService> = OnceCell::const_new();

pub async fn get_valkey_service() -> &'static ValkeyService {
    VALKEY_SERVICE
        .get_or_init(|| async {
            ValkeyService::new()
                .await
                .expect("Failed to create service")
        })
        .await
}

impl ValkeyService {
    pub async fn new() -> redis::RedisResult<Self> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        Ok(Self { client })
    }

    pub async fn set_type(&self, key: &str, value: &str) -> redis::RedisResult<()> {
        use redis::AsyncCommands;
        let mut con = self.client.get_async_connection().await?;
        con.set_ex(key, value, 60).await
    }

    pub async fn get_key(&self, key: &str) -> redis::RedisResult<Option<String>> {
        use redis::AsyncCommands;
        let mut con = self.client.get_async_connection().await?;
        con.get(key).await
    }
}
