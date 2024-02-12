pub mod receiver;
pub mod sender;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct SurServer {
    /// Your slack bot token for api requests
    token: String,
    client: reqwest::Client,
    /// Data structure to track user acknowledgements on messages
    ack: Arc<RwLock<HashMap<(String, String), i64>>>,
}

impl SurServer {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::new(),
            ack: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
