use std::time::Duration;
use crate::{ws::WsConn};
use crate::lobby::Lobby;
use crate::structs::{self, Info};
use actix::Addr;
use tokio;
use actix_web::{get, post, web, web::Data, Responder, Result, web::Path, web::Payload, Error, HttpResponse, HttpRequest};
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

fn _my_handler() -> impl Responder {
    // <-- Bad practice! Will cause the current worker thread to hang!
    std::thread::sleep(Duration::from_secs(5));
    "response"
}

async fn _my_handler2() -> impl Responder {
    // <-- Ok. Worker thread will handler other requests here
    tokio::time::sleep(Duration::from_secs(5)).await;
    "response"
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

pub async fn app_name_state(data: Data<structs::AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <-- response with app_name

    // "index.html"
    // HttpResonse::Ok().body("index.html")
}

#[get("/counter_state")]
async fn counter_state(data: Data<structs::AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

// extract path info from "/users/{user_id}/{friend}" url
// {user_id} - deserializes to a u32
// {friend} - deserializes to a String
// Path<(u32, String)> - tuple type
#[get("/users/{user_id}/{friend}")]
async fn users(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}", friend, user_id))
}

// extract path info using serde
#[get("/users/type/{user_id}/{friend}")]
async fn serde_type(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", info.friend, info.user_id))
}

// non-type-safe alternative
#[get("/users/unsafe/{user_id}/{friend}/")]
async fn unsafe_users(req:HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: u32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 bad request error response is returned
#[get("/welcome")]
async fn welcome(info: web::Query<structs::Info>) -> String {
    format!("Welcome {}, user_id {}", info.friend, info.user_id)
}