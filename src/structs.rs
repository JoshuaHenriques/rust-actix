use std::sync::Mutex;
use serde::Deserialize;
pub struct AppState {
    pub app_name: String,
}

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[derive(Deserialize)]
pub struct Info {
    pub user_id: u32,
    pub friend: String,
}