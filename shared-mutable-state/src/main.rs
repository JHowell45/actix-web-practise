use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // Mutex is necessary to mutate safetly across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with counter
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // NOTE: web::Data created _outside_ of the HttpServer::new closure
    // If created inside it risks being de-synced if modified, so must be created outside and cloned/moved in
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0)
    });

    HttpServer::new(|| {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register created data
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}