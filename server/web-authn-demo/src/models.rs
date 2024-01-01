use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{HttpResponse, HttpResponseBuilder};
use ring::digest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub credId: String,
    pub pubKey: String,
}

impl User {
    pub fn new(credential_id: String, public_key: String) -> User {
        User {
            credId: credential_id,
            pubKey: public_key,
        }
    }
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
    pub credentialId: String,
    pub userId: String,
    pub clientDataJson: ClientDataJson,
}

#[derive(Deserialize, Debug)]
pub struct VerifyPublicKeyReq {
    pub clientDataJson: String,
    pub signature: String,
    pub authenticatorData: String,
    pub userHandle: String,
}

#[derive(Deserialize, Debug)]
pub struct ClientDataJson {
    pub challenge: String,
    pub origin: String,
    #[serde(rename = "type")]
    pub t: String,
    pub androidPackageName: Option<String>,
}

pub enum WebAuthnType {
    CREATE,
    GET,
}

impl ToString for WebAuthnType {
    fn to_string(&self) -> String {
        match self {
            WebAuthnType::CREATE => "webauthn.create".to_owned(),
            WebAuthnType::GET => "webauthn.get".to_owned(),
        }
    }
}

impl ClientDataJson {
    /// Validates the client data json object by checking the incoming challenge and that
    /// the correct operation is being performed
    pub fn validate(
        &self,
        challenge_map: &mut HashMap<String, String>,
        incoming_challenge_id: &String,
        webauthn_type: WebAuthnType,
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
        if self.t != webauthn_type.to_string() {
            return Err(HttpResponse::BadRequest());
        }
        Ok(())
    }

    pub fn sha_256_hash(&self) -> Vec<u8> {
        let mut context = digest::Context::new(&digest::SHA256);

        let json_str = format!(
            r#"{{"challenge":"{}","origin":"{}","type":"{}"}}"#,
            self.challenge, self.origin, self.t
        );

        context.update(json_str.as_bytes());

        context.finish().as_ref().to_vec()
    }
}

pub struct AppData {
    pub userDb: Arc<Mutex<HashMap<String, User>>>,
    pub challenge_map: Arc<Mutex<HashMap<String, String>>>,
}

impl AppData {
    pub fn init() -> AppData {
        AppData {
            userDb: Arc::new(Mutex::new(HashMap::new())),
            challenge_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
