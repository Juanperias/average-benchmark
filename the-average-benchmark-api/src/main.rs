use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

mod rate_limiter;
mod models;
mod handlers;

use rate_limiter::RateLimiterMiddleware;
use handlers::submit_tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Rate limiter setting: 5 requests per hour
    let rate_limiter = RateLimiterMiddleware::new(5, 3600);

    HttpServer::new(move || {
        App::new()
            .wrap(rate_limiter.clone())
            .route("/submit-tests", web::post().to(submit_tests))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
