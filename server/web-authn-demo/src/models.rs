use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    userId: String,
    credId: String,
    pubKey: String,
}

#[derive(Deserialize, Debug)]
pub struct SavePublicKeyReq {
    publicKey: String,
}

pub struct AppData {
    userDb: Arc<Mutex<Vec<User>>>,
}

impl AppData {
    pub fn init() -> AppData {
        AppData {
            userDb: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
