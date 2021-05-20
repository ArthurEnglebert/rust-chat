use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use actix::*;
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use clap::{App as ClapApp, Arg};

mod server;
mod session;
mod db;

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession::new(srv.get_ref().clone()),
        &req,
        stream,
    )
}

///  Displays and affects state
async fn get_count(count: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let current_count = count.fetch_add(1, Ordering::SeqCst);
    format!("Visitors: {}", current_count)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
                .default_value("8081")
        ).get_matches();

    let port = app.value_of("port").unwrap();
    println!("port : {}", port);

    let connection = db::connection::establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }

    // App state
    // We are keeping a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // Start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();

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
            // .service(fs::Files::new("/static/", "static/"))
    })
        .bind(format!("{}:{}", "127.0.0.1", port))?
        .run()
        .await
}