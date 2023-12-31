mod ws;
mod messages;
mod lobby;
mod structs;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::cell::Cell;
use lobby::Lobby;
mod routes;
use actix::Actor;

use actix_web::{App, HttpServer, web, error, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let chat_server = Lobby::default().start();

    // Note: web::Data created _outside_ HttpServer::new closure
    // State initialized inside the closure passed to HttpServer::new is local to the worker thread and may become de-synced if modified
    // To achieve globally shared state, it must be created outside of the closure passed to HttpServer::new and moved/cloned in
    let mut_counter = web::Data::new(structs::AppStateWithCounterMutex {
        mut_counter: Mutex::new(0),
    });

    let cell_counter = structs::AppStateWithCounter {
        cell_counter: Cell::new(0),
        arc_counter: Arc::new(AtomicUsize::new(0)),
    };

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 - newKey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();


    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });

        App::new()
            // setting up routes
            .service(routes::start_connection)
            .service(routes::manual_hello)
            .service(routes::echo)
            .service(routes::hello)
            .service(routes::counter_state)
            .service(routes::counter_state2)
            .service(routes::users)
            .service(routes::unsafe_users)
            .service(routes::welcome)
            .service(routes::serde_type)
            // manual way to assign a route
            // .route("/hey", web::get().to(routes::manual_hello))
            .service(
                // This scope represents a resource prefix that will be prepended to all resource patterns added by the resource configuration.
                web::scope("/app")
                    .route("/name", web::get().to(routes::app_name_state))
            )

            // data and app_data are similar for adding state
            .data(chat_server.clone())
            .app_data(web::Data::new(structs::AppState {
                app_name: String::from("Actix Web"),
                mut_name: String::from("Mutate Me"),
            }))
            .app_data(json_config)
            .app_data(mut_counter.clone()) // <- register the created data
            .app_data(cell_counter.clone()) // <- register the created data

    })
        // .bind_openssl("127.0.0.1:8080", builder)?
        .bind("127.0.0.1:8080")?
        // .workers(4);
        .run()
        .await
}
