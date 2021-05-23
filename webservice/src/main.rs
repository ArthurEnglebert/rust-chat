#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod websocket;

use dotenv::dotenv;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use actix::*;
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use clap::{App as ClapApp, Arg};
use db::messages::MessageRepository;
use db::clients::ClientRepository;
use db::canals::CanalRepository;

pub struct AppState {
    pub visitor_count: AtomicUsize,
    pub message_repo: Box<dyn MessageRepository + Send + Sync>,
    pub client_repo: Box<dyn ClientRepository + Send + Sync>,
    pub canal_repo: Box<dyn CanalRepository + Send + Sync>,
}

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<websocket::server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        websocket::session::WsChatSession::new(srv.get_ref().clone()),
        &req,
        stream,
    )
}

///  Displays and affects state
async fn get_count(app_state: web::Data<AppState>) -> impl Responder {
    let current_count = app_state.visitor_count.fetch_add(1, Ordering::SeqCst);
    format!("Visitors: {}", current_count)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("cannot load dotenv");

    env_logger::init();

    let app = ClapApp::new("Rust Chat Server")
        .version("1.0")
        .author("Arthur Englebert <arthur.englebert@skynet.be>")
        .about("just testing things around with rust")
        .arg(
            Arg::with_name("port")
                .index(1)
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("The port to bind the server on")
                .default_value("8084")
        ).get_matches();

    let port = app.value_of("port").unwrap();
    println!("port : {}", port);

    let pool = db::connection::establish_pool();
    let message_repo = db::messages::MessageRepositoryFactory::new(Some(pool.clone()));
    let client_repo = db::clients::MysqlClientRepository::new(pool.clone());
    let canal_repo = db::canals::MysqlCanalRepository::new(pool.clone());

    // App state
    // We are keeping a count of the number of visitors
    let app_state = Arc::new(AppState {
        visitor_count: AtomicUsize::new(0),
        message_repo: Box::new(message_repo),
        client_repo: Box::new(client_repo),
        canal_repo: Box::new(canal_repo),
    });

    // Start chat server actor
    let server = websocket::server::ChatServer::new(app_state.clone()).start();

    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .data(server.clone())
            // redirect to websocket.html
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/static/websocket.html")
                    .finish()
            })))
            .route("/count/", web::get().to(get_count))
            // websocket
            .service(web::resource("/ws/").to(chat_route))
            // static resources
            .service(fs::Files::new("/static/", "static/"))
    })
        .bind(format!("{}:{}", "127.0.0.1", port))?
        .run()
        .await
}