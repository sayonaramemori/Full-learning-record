use std::sync::Arc;
use std::fmt::Write;
use opcua::client::prelude::*;
use opcua::sync::*;
use crate::ToRedis::Models::RedisData;
use crate::ToRedis::MonitoredItemConfig::MonitorItemConfig;
pub mod ToRedis;


pub fn create_session(endpoint_url:&str) -> Arc<RwLock<Session>>{
    let mut client = ClientBuilder::new()
        .application_name("My First Client")
        .application_uri("urn:MyFirstClient")
        .create_sample_keypair(true)
        .trust_server_certs(false)
        .session_retry_limit(3)
        .client().unwrap();
    // Create an endpoint. The EndpointDescription can be made from a tuple consisting of
    // the endpoint url, security policy, message security mode and user token policy.
    let endpoint: EndpointDescription = (endpoint_url, "None", MessageSecurityMode::None, UserTokenPolicy::anonymous()).into();
    // Create the session
    let session = client.connect_to_endpoint(endpoint, IdentityToken::Anonymous).expect("connect failed");
    return session;
}


pub fn do_test(session: Arc<RwLock<Session>>){
    // Create a subscription and monitored items
    let item1 = vec!["模拟器示例.函数.Ramp1".to_string()];
    let item2 = vec!["ForOPCUA.PLC.temperature".to_string()];
    let config_ramp = MonitorItemConfig::new(item1, to_redis_temp, 1000.0);
    let config_temp= MonitorItemConfig::new(item2, to_redis_temp, 1000.0);
    let redis_data = RedisData::new("redis://kazusa.vip/".to_string(), "Iloveyouxuwu121234".to_string());
    let reference = Arc::new(redis_data);
    if 
    // subscribe_to_values(session.clone(),config_ramp,reference.clone()).is_ok() 
    // &&
    subscribe_to_values(session.clone(),config_temp,reference.clone()).is_ok()
    {
        let _ = Session::run(session);
    } else {
        println!("Error creating subscription");
    }
}

fn subscribe_to_values(session: Arc<RwLock<Session>>,config: MonitorItemConfig,redis_data:Arc<RedisData>) -> Result<u32, StatusCode> 
{
    let session = session.read();
    // Create a subscription polling every 2s with a callback
    let subscription_id = session.create_subscription(config.publish_interval, 10, 30, 0, 0, true, DataChangeCallback::new(move |changed_monitored_items| {
        changed_monitored_items.iter().for_each(|item| {
            (config.call_back)(item,redis_data.clone());
        });
    }))?;
    // Create some monitored items
    let items_to_create: Vec<MonitoredItemCreateRequest> = config.items.clone().into_iter()
        .map(|v| NodeId::new(2, v).into()).collect();
    let _ = session.create_monitored_items(subscription_id, TimestampsToReturn::Source, &items_to_create)?;
    Ok(subscription_id)
}

fn to_redis_temp(item:&MonitoredItem,redis_data:Arc<RedisData>) {
    let data_value = item.last_value();
//    let node_id = &item.item_to_monitor().node_id;
    if let Some(ref value) = data_value.value {
        let val = value.to_string();
        let time = data_value.source_timestamp.unwrap().as_chrono().naive_local() + chrono::Duration::hours(8);
        let mut res = String::new();
        let _ = write!(res,"{}|{:?}",val,time);
        println!("{}",res);
        let _ = redis_data.as_ref().rpush("record", res);
    }
}
