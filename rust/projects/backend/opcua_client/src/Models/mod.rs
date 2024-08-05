pub mod MonitoredItemConfig;
pub mod Temperature;

use crate::store::RedisData::RedisData;
use std::sync::Arc;
use opcua::client::prelude::*;
pub trait ToRedis {
    fn to_redis(item:&MonitoredItem,redis_data:Arc<RedisData>);
}