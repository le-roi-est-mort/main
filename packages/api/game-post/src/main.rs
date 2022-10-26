
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use service

#[derive(Deserialize)]
struct Request {
    player_id: String,
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
    let name = payload.name;
    
    let resp = Response {
        req_id: context.request_id,
        msg: format!("Hello {}!", name),
    };

    Ok(resp)
}
