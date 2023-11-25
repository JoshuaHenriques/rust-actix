use std::sync::Mutex;
use serde::Deserialize;
use std::cell::Cell;
pub struct AppState {
    pub app_name: String,
    pub mut_name: String,
}

pub struct AppStateWithCounterMutex {
    pub mut_counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[derive(Clone)]
pub struct AppStateWithCounterCell{
    pub cell_counter: Cell<usize>, // <- Mutex is necessary to mutate safely across threads
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