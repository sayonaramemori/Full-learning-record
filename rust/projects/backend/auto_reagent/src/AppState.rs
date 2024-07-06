use std::sync::Arc;

#[derive(Clone)]
pub struct RedisState {
    pub redis_client: Arc<redis::Client>,
    pub redis_passwd: String,
}