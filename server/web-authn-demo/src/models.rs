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
    pub publicKey: String,
    pub userId: String,
}

#[derive(Deserialize, Debug)]
pub struct Credentials {
    id: String,
    rawId: Vec<u8>,
    response: CredientialsResponse,
}

#[derive(Deserialize, Debug)]
pub struct CredientialsResponse {
    clientDataJSON: Vec<u8>,
    authenticatorData: Vec<u8>,
    signature: Vec<u8>,
    userHandle: Vec<u8>,
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
