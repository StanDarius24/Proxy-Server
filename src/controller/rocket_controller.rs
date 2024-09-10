use rocket::get;
use rocket::routes;

pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}