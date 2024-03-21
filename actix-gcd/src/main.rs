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

mod gcd_module;
use gcd_module::gcd_module::gcd;
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring!!");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {}  is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );
}
