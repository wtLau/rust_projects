use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    x: i32,
    y: i32,
}

#[derive(Serialize)]
struct Response {
    total: i32,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract numbers from the event
    let x = event.payload.x;
    let y = event.payload.y;

    // Add the numbers
    let total = x + y;

    // Prep the resp
    let resp = Response { total };

    // Return rpose, serialized to json in runtime
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the mdule in every log line
        .with_target(false)
        // disable time. Its handy because CloudWatch will add the ingestion time
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
