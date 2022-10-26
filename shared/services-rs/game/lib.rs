use aws_sdk_dynamodb::Client;
use uuid::Uuid;


pub struct Game {
    id: String,
    name: String,
}

pub struct DynamoGame {
    HK: String,
    SK: String,
}

pub fn createGame(player: Player) {
    let uuid = Uuid::new_v4().to_string();
}
