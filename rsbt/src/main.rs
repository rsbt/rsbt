/*!
# rust description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rust = "0.1"
```

*/

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
