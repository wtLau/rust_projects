use std::sync::Mutex;

use actix_web::{
    web::{self, get},
    App, HttpServer,
};

struct AppStateWithCounter {
    counter: Mutex<i32>, // Mutex is needed to safely mutate across thread
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // get counter's MutexGuard
    *counter += 1; // access counter inside mutexguard

    format!("Request number:{counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Created counter outside of closure to have shares global state
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // register the created data
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// Key takeaways:
// State initialized inside the closure passed to HttpServer::new is local to the worker thread and may become de-synced if modified.
// To achieve globally shared state, it must be created outside of the closure passed to HttpServer::new and moved/cloned in.
