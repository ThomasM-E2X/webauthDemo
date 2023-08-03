use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    userId: String,
    credId: String,
    pubKey: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GenerateChallengeRes {
    challenge: Vec<u8>,
    challenge_id: String,
}

impl GenerateChallengeRes {
    pub fn new(id: String, challenge: Vec<u8>) -> GenerateChallengeRes {
        GenerateChallengeRes {
            challenge: challenge,
            challenge_id: id,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SavePublicKeyReq {
    pub publicKey: String,
    pub id: String,
    pub clientDataJson: ClientDataJson,
}

#[derive(Deserialize, Debug)]
pub struct VerifySignatureReq {
    pub authenticatorData: String,
    pub signature: String,
    pub clientDataJson: ClientDataJson,
}

#[derive(Deserialize, Debug)]
pub struct ClientDataJson {
    pub challenge: String,
    pub origin: String,
    #[serde(rename = "type")]
    pub t: String,
}

pub struct AppData {
    pub userDb: Arc<Mutex<Vec<User>>>,
    pub challenge_map: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl AppData {
    pub fn init() -> AppData {
        AppData {
            userDb: Arc::new(Mutex::new(Vec::new())),
            challenge_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

// impl PartialEq for Vec<u8> {
//     fn eq(&self, other: &Self) -> bool {
//         let matching = self
//             .iter()
//             .zip(other.iter())
//             .filter(|&(a, b)| a == b)
//             .count();

//         return matching != self.len() && matching != other.len();
//     }

//     fn ne(&self, other: &Self) -> bool {
//         !self.eq(other)
//     }
// }
