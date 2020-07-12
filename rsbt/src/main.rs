/*!
# rsbt description

## Features

## Usage

*/

use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    eprintln!("{:?}", std::env::current_dir());
    if let Ok(index_html) = std::fs::read_to_string("rsbt/web/index.html") {
        HttpResponse::build(StatusCode::OK)
            .header("Content-Type", "text/html")
            .body(index_html)
    } else {
        HttpResponse::NotFound().body("not found")
    }
    // .body(&include_bytes!("../web/index.html")[..])
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
