use super::Models::RedisData;
use opcua::client::prelude::*;
use std::sync::Arc;

pub struct MonitorItemConfig {
    pub items: Vec<String>,
    pub call_back: fn(&MonitoredItem,Arc<RedisData>),
    pub publish_interval: f64,
}
impl MonitorItemConfig {
    pub fn new(items:Vec<String>,call_back:fn(&MonitoredItem,Arc<RedisData>),publish_interval:f64) -> MonitorItemConfig {
        MonitorItemConfig {items,call_back,publish_interval}
    }
}