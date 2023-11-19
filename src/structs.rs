use std::sync::Mutex;

pub struct AppState {
    pub app_name: String,
}

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}