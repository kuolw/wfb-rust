use ntex::web::{self, App, HttpRequest};
use rand::Rng;

async fn index(_: HttpRequest) -> String {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(1000..=9999);
    format!("Hello Ntex. #{}", rand)
}

#[ntex::main]
pub async fn main() -> std::io::Result<()> {
    web::server(|| {
        App::new()
            .service((
                web::resource("/").to(index),
            ))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}