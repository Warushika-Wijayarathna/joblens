mod ai;
mod models;
mod routes;
mod utils;
mod db;

use actix_web::{App, HttpServer, Responder, get};

#[get("/")]
async fn hello() -> impl Responder {
    "JobLens Backend Running!!!"
}

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

