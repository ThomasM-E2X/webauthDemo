use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{HttpResponse, HttpResponseBuilder};
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

impl ClientDataJson {
    /// Validates the client data json object by checking the incoming challenge and that
    /// the correct operation is being performed
    pub fn validate(
        &self,
        challenge_map: &mut HashMap<String, String>,
        incoming_challenge_id: &String,
    ) -> Result<(), HttpResponseBuilder> {
        //Make sure incoming challenge matches one we sent out for that session
        match challenge_map.get(incoming_challenge_id) {
            Some(challenge) => {
                if self.challenge != *challenge {
                    return Err(HttpResponse::Unauthorized());
                }
            }
            None => {
                return Err(HttpResponse::NotFound());
            }
        }

        challenge_map.remove(incoming_challenge_id);

        // check is the correct operation
        if (self.t != "webauthn.create") {
            return Err(HttpResponse::BadRequest());
        }
        Ok(())
    }
}

pub struct AppData {
    pub userDb: Arc<Mutex<Vec<User>>>,
    pub challenge_map: Arc<Mutex<HashMap<String, String>>>,
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
