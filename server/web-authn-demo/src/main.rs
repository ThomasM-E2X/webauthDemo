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

    match res
        .clientDataJson
        .validate(&mut challenge_map, &incoming_challenge_id)
    {
        Ok(_) => HttpResponse::Ok(),
        Err(http_response) => http_response,
    }
}

#[post("/verify_public_key")]
async fn verify_public_key(res: web::Json<SavePublicKeyReq>) -> impl Responder {
    println!("{:?}", res);

    ""
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
