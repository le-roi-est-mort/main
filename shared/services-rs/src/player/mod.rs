pub mod errors;
pub mod structs;

use crate::{dynamo::CLIENT, get_ttl};

use aws_sdk_dynamodb::model::AttributeValue;

pub enum PlayerDynamoRequestError {
    NotFound,
    MissingProp(),
}

pub fn player_hk(game_id: &String) -> String {
    return format!("GAME#{}", game_id);
}

pub fn player_sk(game_id: &String, player_id: &String) -> String {
    return format!("GAME#{}#PLAYER#{}", game_id, player_id);
}

pub async fn create_player(input: structs::CreatePlayerInput) -> errors::Result<structs::Player> {
    let player = structs::Player {
        game_id: input.game_id,
        player_id: input.player_id,
        username: input.username,
    };
    let request = CLIENT
        .get()
        .await
        .put_item()
        .table_name("game")
        .item("hk", AttributeValue::S(player_hk(&player.game_id)))
        .item(
            "sk",
            AttributeValue::S(player_sk(&player.game_id, &player.player_id)),
        )
        .item("game_id", AttributeValue::S(String::from(&player.game_id)))
        .item(
            "player_id",
            AttributeValue::S(String::from(&player.player_id)),
        )
        .item(
            "username",
            AttributeValue::S(String::from(&player.username)),
        )
        .item("ttl", AttributeValue::N(get_ttl!()));

    request.send().await?;

    Ok(player)
}

pub async fn get_player(game_id: String, player_id: String) -> errors::Result<structs::Player> {
    let request = CLIENT
        .get()
        .await
        .get_item()
        .table_name("game")
        .key("hk", AttributeValue::S(player_hk(&game_id)))
        .key("sk", AttributeValue::S(player_sk(&game_id, &player_id)));

    let reslut = request.send().await?;
    let item = reslut.item().ok_or(errors::DynamoRequestError::NotFound)?;

    Ok(structs::Player {
        game_id: item
            .get("game_id")
            .ok_or(errors::DynamoRequestError::MissingProp(
                errors::MissingPropError {
                    id: Some(format!("{}, {}", game_id, player_id)),
                    prop_name: Some("game_id".to_string()),
                },
            ))?
            .as_s()
            .or_else(|res| {
                Err(errors::WrongPropTypeError {
                    id: Some(format!("{}, {}", game_id, player_id)),
                    prop_name: Some("game_id".to_string()),
                    prop_intended_type: Some("S".to_string()),
                    prop_value: Some(res.clone()),
                })
            })?
            .to_string(),
        player_id: item
            .get("player_id")
            .ok_or(errors::MissingPropError {
                id: Some(format!("{}, {}", game_id, player_id)),
                prop_name: Some("player_id".to_string()),
            })?
            .as_s()
            .or_else(|res| {
                Err(errors::WrongPropTypeError {
                    id: Some(format!("{}, {}", game_id, player_id)),
                    prop_name: Some("player_id".to_string()),
                    prop_intended_type: Some("S".to_string()),
                    prop_value: Some(res.clone()),
                })
            })?
            .to_string(),
        username: item
            .get("username")
            .ok_or(errors::MissingPropError {
                id: Some(format!("{}, {}", game_id, player_id)),
                prop_name: Some("username".to_string()),
            })?
            .as_s()
            .or_else(|res| {
                Err(errors::WrongPropTypeError {
                    id: Some(format!("{}, {}", game_id, player_id)),
                    prop_name: Some("username".to_string()),
                    prop_intended_type: Some("S".to_string()),
                    prop_value: Some(res.clone()),
                })
            })?
            .to_string(),
    })
}
