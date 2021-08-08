use rocket::{get, launch, routes};
use rand::Rng;

#[get("/")]
fn index() -> String {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(1000..=9999);
    format!("Hello Ntex. #{}", rand)
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}