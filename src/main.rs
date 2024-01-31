use actix_web::{App, HttpServer};
use docker_calculator::{handle_add, handle_div, handle_mul, handle_sub};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_add)
            .service(handle_sub)
            .service(handle_mul)
            .service(handle_div)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
