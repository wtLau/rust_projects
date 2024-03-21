use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    let app = || App::new().route("/", web::get().to(get_index));

    let server = HttpServer::new(app)
        .bind("127.0.0.1:8080")
        .expect("Error bindign server to address");

    println!("Serving on http://localhost:8080....");

    server.run().await.expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title> GCD Calculator</title>
            <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit"> Compute GCD</button>
            </form>
            "#,
    )
}

use serde::Deserialize;
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}
