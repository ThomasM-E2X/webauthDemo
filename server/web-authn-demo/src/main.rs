mod models;

use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, Bytes, Data},
    App, HttpResponse, HttpServer, Responder,
};

use base64::{engine::general_purpose, Engine as _};

use rand::thread_rng;
use rand::Rng;

use crate::models::{AppData, GenerateChallengeRes, SavePublicKeyReq};

#[get("/generate_challenge")]
async fn generate_challenge(data: web::Data<AppData>) -> impl Responder {
    let mut challenge_map = data
        .challenge_map
        .try_lock()
        .expect("failed to get a lock ");

    let id = uuid::Uuid::new_v4().to_string();

    let mut arr = [0u8; 16];
    thread_rng().try_fill(&mut arr[..]).expect("Ooops!");

    challenge_map.insert(id.clone(), arr.into());

    return web::Json(GenerateChallengeRes::new(id.clone(), arr.to_vec()));
}

#[post("/save_public_key/{challenge_id}")]
async fn save_public_key(
    path: web::Path<String>,
    res: web::Json<SavePublicKeyReq>,
    data: web::Data<AppData>,
) -> impl Responder {
    println!("{:?}", res.publicKey);

    //Make sure incoming challenge matches challenge we sent
    let challenge_id = path.into_inner();
    let mut challenge_map = data.challenge_map.lock().expect("failed to lock");
    let incoming_challenge = res.clientDataJson.challenge.as_bytes();

    match challenge_map.get(&challenge_id) {
        Some(challenge) => {
            let matching = challenge
                .iter()
                .zip(incoming_challenge.iter())
                .filter(|&(a, b)| a == b)
                .count();

            if (matching != challenge.len() || matching != incoming_challenge.len()) {
                return HttpResponse::Unauthorized();
            }
        }
        None => {
            return HttpResponse::NotFound();
        }
    }

    challenge_map.remove(&challenge_id);

    // check is the correct operation
    if (res.clientDataJson.t != "webauthn.create") {
        return HttpResponse::BadRequest();
    }

    //Save public key
    HttpResponse::Ok()
}

#[post("/verify_public_key")]
async fn verify_public_key(res: web::Json<SavePublicKeyReq>) -> impl Responder {
    println!("{:?}", res);

    ""
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .app_data(Data::new(AppData::init()))
            .wrap(cors)
            .service(generate_challenge)
            .service(save_public_key)
            .service(verify_public_key)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
