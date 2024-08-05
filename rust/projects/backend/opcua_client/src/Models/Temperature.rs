use std::convert::From;
use redis::{Commands, ConnectionLike, RedisError, RedisResult};
use std::sync::{self,Arc};
use chrono::prelude::{DateTime,NaiveDateTime,*};
use opcua::client::prelude::*;
use std::fmt::Write;
use crate::store::RedisData::RedisData;
use crate::Models::ToRedis;
// use crate::opcua_config::Operation::DataTime;
use crate::store::opcuaSession::DataTime;


#[derive(Default,Debug)]
pub struct Temperature {
    pub val: f64,
    pub time: NaiveDateTime,
}

impl ToRedis for Temperature{
    fn to_redis(item:&MonitoredItem,redis_data:Arc<RedisData>) {
        let data_value = item.last_value();
    //    let node_id = &item.item_to_monitor().node_id;
        if let Some(ref value) = data_value.value {
            let val = value.convert(VariantTypeId::Double);
            let time = data_value.source_timestamp.unwrap().as_chrono();
            let offset = FixedOffset::east_opt(8*3600).unwrap();
            let time = time.with_timezone(&offset);
            let mut res = String::new();
            let _ = write!(res,"{}|{:?}",val,time);
            println!("{}",res);
            if !redis_data.as_ref().rpush("record", res.clone()) {
                redis_data.as_ref().rpush("record", res);
            }
        }else {
            println!("Read value failed");
        }
    }
}

impl From<String> for Temperature{
    fn from(value: String) -> Self {
        let mut res = vec![];
        let _ = value.split('|').map(|val|{res.push(val.to_string());}).last();
        if res.len()==2 {
            let format = "%Y-%m-%dT%H:%M:%S%.f%:z";
            if let Ok(time) = DateTime::parse_from_str(&res[1],format) {
                let res = Temperature{val:res[0].parse::<f64>().unwrap_or(0.0),time:time.naive_local()};
                return res;
            }
        }
        Temperature::default()
    }
}

impl From<DataTime> for Temperature {
    fn from(value: DataTime) -> Self {
        Temperature {
            val: value.data.parse::<f64>().unwrap_or(0.0),
            time: value.time.naive_local(),
        }
    }
}
