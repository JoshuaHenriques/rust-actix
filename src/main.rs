mod ws;
mod messages;
mod lobby;
mod structs;
use std::sync::Mutex;

use lobby::Lobby;
mod routes;
use actix::Actor;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let chat_server = Lobby::default().start();

    // Note: web::Data created _outside_ HttpServer::new closure
    // State initialized inside the closure passed to HttpServer::new is local to the worker thread and may become de-synced if modified
    // To achieve globally shared state, it must be created outside of the closure passed to HttpServer::new and moved/cloned in
    let counter = web::Data::new(structs::AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            // setting up routes
            .service(routes::start_connection)
            .service(routes::manual_hello)
            .service(routes::echo)
            .service(routes::hello)
            .service(routes::index_state)

            // manual way to assign a route
            // .route("/hey", web::get().to(routes::manual_hello))
            .service(
                // This scope represents a resource prefix that will be prepended to all resource patterns added by the resource configuration.
                web::scope("/app")
                    .route("/index", web::get().to(routes::index))
            )

            // data and app_data are similar for adding state
            .data(chat_server.clone())
            .app_data(web::Data::new(structs::AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(counter.clone()) // <- register the created data

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
