// use actix_web::{
//     body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
// };
use std::{
    cell::Cell,
    sync::atomic::AtomicUsize,
    sync::Arc,
    sync::Mutex,
};
use serde::{Deserialize, Serialize};
pub struct AppState {
    pub app_name: String,
    pub mut_name: String,
}

pub struct AppStateWithCounterMutex {
    // <- Mutex is necessary to mutate safely across threads
    // Mutex blocks the current the current thread    
    pub mut_counter: Mutex<i32>, 
}

#[derive(Clone)]
pub struct AppStateWithCounter {
    // will only count th enumber of requests handled by each worker thread
    pub cell_counter: Cell<usize>, 
    // to count the number of total requests across all threads
    pub arc_counter: Arc<AtomicUsize>,
}

#[derive(Deserialize)]
pub struct Info {
    pub user_id: u32,
    pub friend: String,
}

#[derive(Deserialize)]
pub struct FormData {
    pub username: String,
}

#[derive(Serialize)]
pub struct MyObj {
    name: &'static str,
}

// Responder
// impl Responder for MyObj {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         let body = serde_json::to_string(&self).unwrap();

//         // create response and set content type
//         HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .body(body)
//     }
// }