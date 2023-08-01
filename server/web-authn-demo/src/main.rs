mod models;

use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, Bytes},
    App, HttpServer, Responder,
};

use base64::{engine::general_purpose, Engine as _};

use rand::thread_rng;
use rand::Rng;

use crate::models::{Credentials, SavePublicKeyReq};

#[get("/generate_challenge")]
async fn generate_challenge() -> impl Responder {
    let mut arr = [0u8; 16];
    thread_rng().try_fill(&mut arr[..]).expect("Ooops!");

    Bytes::copy_from_slice(&arr)
}

#[post("/save_public_key")]
async fn save_public_key(res: web::Json<SavePublicKeyReq>) -> impl Responder {
    println!("{:?}", res.publicKey);

    //Save...
    ""
}

#[post("/verify_public_key")]
async fn verify_public_key(res: web::Json<Credentials>) -> impl Responder {
    println!("{:?}", res);

    ""
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(generate_challenge)
            .service(save_public_key)
            .service(verify_public_key)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
