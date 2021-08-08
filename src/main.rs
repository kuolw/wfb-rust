mod actix;
mod tide;
mod ntex;
mod rocket;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Requests per second:    51637.01 [#/sec] (mean)
    fn actix() {
        actix::main();
    }

    #[test]
    // Requests per second:    2716.43 [#/sec] (mean)
    fn tide() {
        tide::main();
    }

    #[test]
    // Requests per second:    50079.01 [#/sec] (mean)
    fn ntex() {
        ntex::main();
    }

    #[test]
    // Requests per second:    9615.33 [#/sec] (mean)
    fn rocket() {
        rocket::main();
    }
}