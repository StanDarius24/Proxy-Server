use tide::Server;
use tide::{Request, Result};

pub async fn set_urls() -> Server<()> {
    let mut app = tide::new();
    app.at("/").get(handle);
    app
}

async fn handle(req: Request<()>) -> Result<String> {
    Ok("Hello, World!".to_string())
}
