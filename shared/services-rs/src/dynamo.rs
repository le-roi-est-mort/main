use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;
use async_once::AsyncOnce;

lazy_static! {
    pub static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
        let region_provider = RegionProviderChain::default_provider().or_else("eu-west-3");
        let config = aws_config::from_env().region(region_provider).load().await;

        Client::new(&config)
    });
}


pub fn get_ttl(ttl: u64) -> String {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + ttl)
        .to_string()
}

#[macro_export]
macro_rules! get_ttl {
    ($a: expr) => {
        $crate::dynamo::get_ttl($a)
    };
    () => {
        $crate::dynamo::get_ttl(86400)
    };
}
