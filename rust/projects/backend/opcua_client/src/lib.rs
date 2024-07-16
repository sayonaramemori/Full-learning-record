use std::sync::{self,Arc};
use opcua::{client::prelude::*, server::callbacks};
use opcua::sync::*;
use redis::{Commands, ConnectionLike, RedisError, RedisResult};
struct MonitorItemConfig {
    items: Vec<String>,
    call_back: fn(&MonitoredItem,Arc<RedisData>),
    publish_interval: f64,
}
impl MonitorItemConfig {
    pub fn new(items:Vec<String>,call_back:fn(&MonitoredItem,Arc<RedisData>),publish_interval:f64) -> MonitorItemConfig {
        MonitorItemConfig {items,call_back,publish_interval}
    }
}
struct RedisData{
    url: String, 
    passwd: String,
    conn: sync::RwLock<Option<redis::Connection>>,
}
impl RedisData{
    fn gain_conn(&self) ->RedisResult<()>{
        let client = redis::Client::open(self.url.as_ref())?;
        let mut current_conn = self.conn.write().unwrap();
        let mut new_conn = client.get_connection()?;
        redis::cmd("Auth").arg(&self.passwd).query::<()>(&mut new_conn)?;
        *current_conn = Some(new_conn);
        Ok(())
    }
    pub fn set(&self, key :&str,data:String){
        let mut conn= self.conn.write().unwrap();
        if let Err(err) = conn.as_mut().unwrap().set::<_,_,()>(key, data) {
            println!("Closed for {}",err);
            drop(conn);
            if let Err(error) = self.gain_conn() {
                println!("Gain connection failed for {}",error);
            }
        };
    }
    pub fn rpush(&self,key: &str,data:String){
        let mut conn= self.conn.write().unwrap();
        if let Err(err) = conn.as_mut().unwrap().rpush::<String,String,()>(key.to_string(), data){
            println!("Closed for {}",err);
            drop(conn);
            if let Err(error) = self.gain_conn() {
                println!("Gain connection failed for {}",error);
            }
        };
    }
    pub fn new(url: String,passwd: String) -> RedisData {
        let res = RedisData{url,passwd,conn:sync::RwLock::new(None)};
        res.gain_conn().unwrap();
        return res;
    }
}

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
    let items = vec!["模拟器示例.函数.Ramp1".to_string(), "ForOPCUA.PLC.temperature".to_string()];
    let config = MonitorItemConfig::new(items, to_redis, 1000.0);
    let redis_data = RedisData::new("redis://kazusa.vip/".to_string(), "Iloveyouxuwu121234".to_string());
    if subscribe_to_values(session.clone(),config,Arc::new(redis_data)).is_ok() {
        let _ = Session::run(session);
    } else {
        println!("Error creating subscription");
    }
}

fn subscribe_to_values(session: Arc<RwLock<Session>>,config: MonitorItemConfig,redis_data:Arc<RedisData>) -> Result<(), StatusCode> 
{
    let mut session = session.read();
    // Create a subscription polling every 2s with a callback
    let subscription_id = session.create_subscription(config.publish_interval, 10, 30, 0, 0, true, DataChangeCallback::new(move |changed_monitored_items| {
        changed_monitored_items.iter().for_each(|item| {
            (config.call_back)(item,redis_data.clone());
        });
    }))?;
    // Create some monitored items
    let items_to_create: Vec<MonitoredItemCreateRequest> = config.items.clone().into_iter()
        .map(|v| NodeId::new(2, v).into()).collect();
    let _ = session.create_monitored_items(subscription_id, TimestampsToReturn::Both, &items_to_create)?;
    Ok(())
}

fn to_redis(item:&MonitoredItem,redis_data:Arc<RedisData>) {
    let data_value = item.last_value();
    if let Some(ref value) = data_value.value {
        let val = value.to_string();
        // println!("{}",val);
        let _ = redis_data.as_ref().set("temp001", val.clone());
        let _ = redis_data.as_ref().rpush("record", val);
    }
}

fn print_value(item: &MonitoredItem) {
   let node_id = &item.item_to_monitor().node_id.to_string();
   let data_value = item.last_value();
   if let Some(ref value) = data_value.value {
        println!("ns:{}, val: {}",node_id,value.to_string());
   } else {
       println!("Item \"{}\", Value not found, error: {}", node_id, data_value.status.as_ref().unwrap());
   }
}