use crate::{ws::WsConn};
use crate::lobby::Lobby;
use crate::structs;
use actix::Addr;
use actix_web::{get, post, web::Data, Responder, web::Path, web::Payload, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use uuid::Uuid;

#[get("/ws/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    Path(group_id): Path<Uuid>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let ws = WsConn::new(
        group_id,
        srv.get_ref().clone()
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/hey")]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn index(data: Data<structs::AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <-- response with app_name

    // "index.html"
    // HttpResonse::Ok().body("index.html")
}

#[get("/index-state")]
async fn index_state(data: Data<structs::AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}