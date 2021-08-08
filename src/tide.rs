use tide::Request;
use rand::Rng;

async fn index(_: Request<()>) -> tide::Result {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(1000..=9999);
    Ok(format!("Hello Tide. #{}", rand).into())
}

#[async_std::main]
pub async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(index);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}