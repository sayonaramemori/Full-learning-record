use std::sync::Mutex;

pub struct AppState {
    pub response: String,
    pub visit_count: Mutex<u32>,
}
