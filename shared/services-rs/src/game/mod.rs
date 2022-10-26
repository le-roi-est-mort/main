use std::collections::HashMap;

use crate::dynamo::CLIENT;
use crate::get_ttl;

use aws_sdk_dynamodb::{
    error::PutItemError, model::AttributeValue, output::PutItemOutput, types::SdkError,
};
use uuid::Uuid;
pub struct GamePlayerHost {
    id: String,
    name: String,
}

pub struct Game {
    id: String,
    name: String,
    host: GamePlayerHost,
}

pub struct DynamoGame {
    hk: String,
    sk: String,
    id: String,
    name: String,
}

pub struct CreateGameInput {
    name: String,
    host_id: String,
}

fn game_hk(id: &String) -> String {
    return format!("GAME#{}", id);
}
fn game_sk(id: &String) -> String {
    return format!("GAME#{}", id);
}

pub async fn create_game(input: CreateGameInput) -> Result<PutItemOutput, SdkError<PutItemError>> {
    let uuid = Uuid::new_v4().to_string();

    let host: HashMap<String, AttributeValue> = HashMap::from([
        (String::from("id"), AttributeValue::S(String::from(&uuid))),
        (String::from("host_id"), AttributeValue::S(input.host_id)),
    ]);

    let request = CLIENT
        .get()
        .await
        .put_item()
        .table_name("game")
        .item("hk", AttributeValue::S(game_hk(&uuid)))
        .item("sk", AttributeValue::S(game_sk(&uuid)))
        .item("game_id", AttributeValue::S(uuid))
        .item("name", AttributeValue::S(String::from(input.name)))
        .item("name", AttributeValue::M(host))
        .item("ttl", AttributeValue::N(get_ttl!()));

    return request.send().await;
}
