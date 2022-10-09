use aws_sdk_dynamodb::Client;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = service_fn(handler);
    run(func).await?;
    Ok(())
}

async fn handler(
    LambdaEvent { payload, context }: LambdaEvent<Request>,
) -> Result<Response, Error> {
    // extract some useful info from the request
    let name = payload.name;

    // prepare the response
    let resp = Response {
        req_id: context.request_id,
        msg: format!("Hello {}!", name),
    };

    Ok(resp)
}
