/*!
# rsbt description

## Features

## Usage

*/

use actix_web::{get, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use futures::FutureExt;
use rsbt_app::{
    request, App as RsbtApp, Command, CommandSender, RsbtResult, TokioMpscSender, TokioTypeFactory,
};

#[post("/api/v1/action")]
async fn api_v1_action(
    sender: web::Data<TokioMpscSender<Command<TokioTypeFactory, RsbtApp<TokioTypeFactory>>>>,
) -> impl Responder {
    let mut sender = sender.as_ref().clone();

    let result = request!(sender, |x: &mut RsbtApp<TokioTypeFactory>| x.toggle());

    HttpResponse::Created()
}

#[get("/")]
async fn index() -> impl Responder {
    #[cfg(feature = "dev")]
    {
        let index_html = std::env::current_dir()
            .unwrap_or_default()
            .join("web/index.html");
        if let Ok(index_html) = std::fs::read_to_string(&index_html) {
            HttpResponse::build(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(index_html)
        } else {
            HttpResponse::NotFound().body(format!("not found {:?}", index_html))
        }
    }
    #[cfg(not(feature = "dev"))]
    {
        HttpResponse::Ok().body(&include_bytes!("../web/index.html")[..])
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let app = RsbtApp::<TokioTypeFactory>::new(Default::default());

    let rsbt_app_sender = app.spawn().await;

    let sender = web::Data::new(rsbt_app_sender);

    HttpServer::new(move || {
        App::new()
            .app_data(sender.clone())
            .service(index)
            .service(api_v1_action)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
