/*!
# rsbt description

## Features

## Usage

*/

use actix_web::{
    dev::Server, get, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder,
};
use env_logger::Builder as LoggerBuilder;
use futures::FutureExt;
use log::{debug, error, info};
use rsbt_app::{
    request, App as RsbtApp, Command, CommandSender, RsbtResult, TokioMpscSender, TokioTypeFactory,
};
use rsbt_web_common::generated_web_common;
use rsbt_web_wizard::generated_web_wizard;
use structopt::StructOpt;
use tokio::sync::{oneshot, Mutex};

mod cli;

#[get("/api/wizard/all-default")]
async fn wizard_all_default(
    quit_trigger: web::Data<Mutex<Option<oneshot::Sender<()>>>>,
) -> impl Responder {
    if let Some(quit_trigger) = quit_trigger.lock().await.take() {
        if let Err(_) = quit_trigger.send(()) {
            return HttpResponse::InternalServerError().finish();
        }
    }

    #[cfg(feature = "dev")]
    {
        response_from_file(
            concat!(module_path!(), "/web/all-default-redirect.html"),
            "text/html",
        )
        .await
    }
    #[cfg(not(feature = "dev"))]
    {
        HttpResponse::Ok()
            .header("Content-Type", "text/html")
            .body(&include_bytes!("../web/all-default-redirect.html")[..])
    }
}

#[post("/api/v1/action")]
async fn api_v1_action(
    sender: web::Data<TokioMpscSender<Command<TokioTypeFactory, RsbtApp<TokioTypeFactory>>>>,
) -> impl Responder {
    let mut sender = sender.as_ref().clone();

    let result = request!(sender, |x: &mut RsbtApp<TokioTypeFactory>| x.toggle());

    HttpResponse::Created()
}

#[cfg(feature = "dev")]
async fn response_from_file(path: &str, content_type: &str) -> HttpResponse {
    let file_path = std::env::current_dir().unwrap_or_default().join(path);
    if let Ok(file_content) = tokio::fs::read_to_string(&file_path).await {
        HttpResponse::build(StatusCode::OK)
            .header("Content-Type", content_type)
            .body(file_content)
    } else {
        HttpResponse::NotFound().body(format!("not found {:?}", file_path))
    }
}

#[get("/")]
async fn index() -> impl Responder {
    #[cfg(feature = "dev")]
    {
        response_from_file(concat!(module_path!(), "/web/index.html"), "text/html").await
    }
    #[cfg(not(feature = "dev"))]
    {
        HttpResponse::Ok()
            .header("Content-Type", "text/html")
            .body(&include_bytes!("../web/index.html")[..])
    }
}

#[actix_rt::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = cli::Cli::from_args();

    cli.verbose
        .log_level()
        .map(|x| {
            LoggerBuilder::new()
                .filter(None, x.to_level_filter())
                .format_timestamp_nanos()
                .try_init()
        })
        .transpose()?;

    info!("RSBT initialization...");

    let need_initial_configuration =
        RsbtApp::<TokioTypeFactory>::check_need_initial_configuration(cli.config_dir.clone())
            .await?;
    info!("Need initial configuration: {}", need_initial_configuration);

    if need_initial_configuration {
        let (quit_trigger_tx, quit_trigger_rx) = oneshot::channel();

        let server_handler: web::Data<Mutex<Option<oneshot::Sender<()>>>> =
            web::Data::new(Mutex::new(Some(quit_trigger_tx)));

        let wizard_listen_addr = cli
            .wizard_listen_addr
            .as_ref()
            .unwrap_or_else(|| &cli.listen_addr);

        let server = HttpServer::new(move || {
            let generated_web_common = generated_web_common();
            let generated_web_wizard = generated_web_wizard();

            debug!("Generated static assets for HTTP worker.");

            App::new()
                .app_data(server_handler.clone())
                .service(wizard_all_default)
                .service(actix_web_static_files::ResourceFiles::new(
                    "/res",
                    generated_web_common,
                ))
                .service(actix_web_static_files::ResourceFiles::new(
                    "/",
                    generated_web_wizard,
                ))
        })
        .bind(wizard_listen_addr)?
        .run();

        let server_clone = server.clone();
        let quit_wizard_task = async move {
            if let Err(err) = quit_trigger_rx.await {
                error!("cannot trigger wizard quit: {}", err);
                return;
            }
            server_clone.stop(true).await;
        };

        tokio::spawn(quit_wizard_task);

        server.await?;
    }

    let app = RsbtApp::<TokioTypeFactory>::new(Default::default());

    let rsbt_app_sender = app.spawn().await;

    let sender = web::Data::new(rsbt_app_sender);

    HttpServer::new(move || {
        App::new()
            .app_data(sender.clone())
            .service(index)
            .service(api_v1_action)
    })
    .bind(cli.listen_addr)?
    .run()
    .await?;

    Ok(())
}
