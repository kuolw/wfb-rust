use actix_web::{web, App, HttpServer, Result};
use rand::Rng;

async fn index() -> Result<String> {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(1000..=9999);
    Ok(format!("Hello Actix. #{}", rand).into())
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}