mod models;

use actix_cors::Cors;
use actix_web::{
    get,
    web::{self, Bytes},
    App, HttpServer, Responder,
};

use rand::thread_rng;
use rand::Rng;

use crate::models::SavePublicKeyReq;

#[get("/generate_challenge")]
async fn generate_challenge() -> impl Responder {
    let mut arr = [0u8; 16];
    thread_rng().try_fill(&mut arr[..]).expect("Ooops!");

    Bytes::copy_from_slice(&arr)
}

#[get("/save_public_key")]
async fn save_public_key(res: web::Json<SavePublicKeyReq>) -> impl Responder {
    println!("{:?}", res.0);

    ""
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(generate_challenge)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
