use std::{
    cell::Cell,
    sync::atomic::AtomicUsize,
    sync::Arc,
    sync::Mutex,
};
use serde::Deserialize;
pub struct AppState {
    pub app_name: String,
    pub mut_name: String,
}

pub struct AppStateWithCounterMutex {
    // <- Mutex is necessary to mutate safely across threads
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