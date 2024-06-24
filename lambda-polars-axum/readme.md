# Build and Invoke Serverless Rust Web Services via AWS Lambda URLs

Learning how to develop Rust AWS Lambdas that expose web URLs for easy remote access. Follow a coding walkthrough using Axum for routes, deploying functions, enabling URLs in the Lambda console, querying the service from the terminal to validate the end-to-end flow - a frictionless way to build and invoke Rust APIs in serverless architectures.

- Built with Axum, polars, Tokio, AWS Lambda

## Test Locally

`make watch`

Open another terminal
`curl localhost:9000`

Should see `Hello, we're using Polars!!`

By route
`curl localhost:9000/iris/filter/4`

## Deploy

`make build & make deploy`
