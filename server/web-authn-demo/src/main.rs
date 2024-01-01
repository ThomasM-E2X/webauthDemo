//TODO: the signature needs to be decoded from asn.1 to something normal

mod models;

use crate::models::{
    AppData, ClientDataJson, GenerateChallengeRes, SavePublicKeyReq, User, VerifyPublicKeyReq,
    WebAuthnType,
};
use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use base64::{engine::general_purpose, Engine as _};
use rand::thread_rng;
use rand::Rng;
use ring::{
    digest,
    signature::{self, UnparsedPublicKey, ECDSA_P256_SHA256_FIXED},
};

#[get("/generate_challenge")]
async fn generate_challenge(data: web::Data<AppData>) -> impl Responder {
    let mut challenge_map = data.challenge_map.lock().expect("failed to get a lock ");

    let id = uuid::Uuid::new_v4().to_string();

    let mut arr = [0u8; 32];
    thread_rng().try_fill(&mut arr[..]).expect("Ooops!");

    challenge_map.insert(id.clone(), general_purpose::URL_SAFE_NO_PAD.encode(&arr));

    return web::Json(GenerateChallengeRes::new(id.clone(), arr.to_vec()));
}

#[post("/save_public_key/{challenge_id}")]
async fn save_public_key(
    path: web::Path<String>,
    res: web::Json<SavePublicKeyReq>,
    data: web::Data<AppData>,
) -> impl Responder {
    let incoming_challenge_id = path.into_inner();
    let mut challenge_map = data.challenge_map.lock().expect("failed to lock");

    match res.clientDataJson.validate(
        &mut challenge_map,
        &incoming_challenge_id,
        WebAuthnType::CREATE,
    ) {
        Ok(_) => match data.userDb.lock() {
            Ok(mut users) => {
                users
                    .entry(res.userId.clone())
                    .or_insert(User::new(res.credentialId.clone(), res.publicKey.clone()));

                HttpResponse::Ok()
            }
            Err(_) => HttpResponse::InternalServerError(),
        },
        Err(http_response) => http_response,
    }
}

#[post("/verify_public_key/{challenge_id}")]
async fn verify_public_key(
    path: web::Path<String>,
    res: web::Json<VerifyPublicKeyReq>,
    data: web::Data<AppData>,
) -> impl Responder {
    let incoming_challenge_id = path.into_inner();
    let mut challenge_map = data.challenge_map.lock().expect("failed to lock");

    let r = general_purpose::URL_SAFE
        .decode(&res.clientDataJson)
        .unwrap();

    let s = String::from_utf8(r).unwrap();

    println!("string {:?}", s);

    let clientData: ClientDataJson = serde_json::from_str(&s).unwrap();

    println!("clientData {:?}", clientData);

    match clientData.validate(
        &mut challenge_map,
        &incoming_challenge_id,
        WebAuthnType::GET,
    ) {
        Ok(_) => match data.userDb.lock() {
            Ok(users) => {
                println!("made it here");
                let user = users
                    .get(&res.userHandle.clone())
                    .expect("failed to get user ");

                let base64PubKey = general_purpose::STANDARD.decode(&user.pubKey).unwrap();

                println!("key {:?}", base64PubKey);
                let public_key = UnparsedPublicKey::new(
                    &ECDSA_P256_SHA256_FIXED,
                    // user.pubKey.bytes().collect::<Vec<u8>>(),
                    base64PubKey,
                );

                let mut context = digest::Context::new(&digest::SHA256);

                context.update(res.clientDataJson.as_bytes());

                let message = [
                    res.authenticatorData.bytes().collect::<Vec<u8>>(),
                    context.finish().as_ref().to_vec(),
                ]
                .concat();

                let result =
                    public_key.verify(&message, &res.signature.bytes().collect::<Vec<u8>>());

                match result {
                    Ok(_) => HttpResponse::Ok(),
                    Err(_) => HttpResponse::Unauthorized(),
                }
            }
            Err(_) => HttpResponse::InternalServerError(),
        },

        Err(http_response) => http_response,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Data::new(AppData::init());

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(data.clone())
            .wrap(cors)
            .service(generate_challenge)
            .service(save_public_key)
            .service(verify_public_key)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
