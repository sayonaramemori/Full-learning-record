use opcua::client::prelude::*;
use std::sync::Arc;
use std::convert::From;
use crate::store::RedisData::RedisData;

pub struct MonitorItemConfig<T>
where T: Fn(&MonitoredItem,Arc<RedisData>) + Send + Sync + 'static
{
    pub items: Vec<String>,
    pub call_back: Box<T>,
    pub publish_interval: f64,
}

impl<T> From<(String,T,f64)> for MonitorItemConfig<T> 
where T: Fn(&MonitoredItem,Arc<RedisData>)+ Send + Sync + 'static
{
    fn from(value: (String,T,f64)) -> Self {
        MonitorItemConfig {
            items:vec![value.0],
            call_back: Box::new(value.1),
            publish_interval: value.2,
        }
    }
}
impl<T> From<(Vec<String>,T,f64)> for MonitorItemConfig<T> 
where T: Fn(&MonitoredItem,Arc<RedisData>)+ Send + Sync + 'static
{
    fn from(value: (Vec<String>,T,f64)) -> Self {
        MonitorItemConfig {
            items:value.0,
            call_back: Box::new(value.1),
            publish_interval: value.2,
        }
    }
}

impl<T> MonitorItemConfig<T> 
where T: Fn(&MonitoredItem,Arc<RedisData>)+ Send + Sync + 'static
{
    pub fn new(items:Vec<String>,call_back:T,publish_interval:f64) -> MonitorItemConfig<T> {
        MonitorItemConfig {items,call_back:Box::new(call_back),publish_interval}
    }
}