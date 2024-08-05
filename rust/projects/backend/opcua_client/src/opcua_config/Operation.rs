use chrono::DateTime;
use chrono::prelude::*;
use opcua::client::prelude::*;
use std::fmt::Display;
use std::sync::Arc;
use opcua::sync::*;
use crate::debug_println;
use crate::utility::time::*;

#[derive(Clone,Debug)]
pub struct DataTime{pub data:String,pub time: DateTime<FixedOffset>}

impl Display for DataTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}|{:?}",self.data,self.time)
    }
}

pub fn create_session(endpoint_url:&str) -> Arc<RwLock<Session>>{
    let mut client = ClientBuilder::new()
        .application_name("My First Client")
        .application_uri("urn:MyFirstClient")
        .create_sample_keypair(true)
        .trust_server_certs(true)
        .session_retry_limit(9)
        // .session_timeout(session_timeout)
        .client().unwrap();
    // Create an endpoint. The EndpointDescription can be made from a tuple consisting of
    // the endpoint url, security policy, message security mode and user token policy.
    let endpoint: EndpointDescription = (endpoint_url, "None", MessageSecurityMode::None, UserTokenPolicy::anonymous()).into();
    // Create the session
    let session = client.connect_to_endpoint(endpoint, IdentityToken::Anonymous).expect("connect failed");
    return session;
}

pub fn read_single(session: Arc<RwLock<Session>>, node_id: &NodeId) -> Option<DataTime>{
    let session = session.read();
    let temp :Vec<ReadValueId>= vec![node_id.into()];
    if let Ok(res) = session.read(&temp, TimestampsToReturn::Both, 0.0){
        for i in res {
            let time = i.server_timestamp.unwrap().as_chrono();
            let time = with_timezone(time);
            if let Some(val) = i.value {
                return Some(DataTime{data:val.to_string(),time});
            }
        }
    }
    None
}

pub fn read_batch(session: Arc<RwLock<Session>>, node_id: &Vec<NodeId>) -> Vec<Option<DataTime>>{
    let session = session.read();
    let temp :Vec<ReadValueId>= node_id.into_iter().map(|v|{ReadValueId::from(v)}).collect();
    match session.read(&temp, TimestampsToReturn::Both, 0.0){
        Ok(value) => {
            let mut res:Vec<Option<DataTime>> = vec![];
            for i in value{
                let time = i.server_timestamp.unwrap().as_chrono();
                let time =with_timezone(time);
                match i.value {
                    Some(val) => res.push(Some(DataTime{data:val.to_string(),time})),
                    _ => res.push(None),
                }
            }
            res
        },
        Err(err) => {
            debug_println!("Read batch failed for {:?}",err);
            return vec![];
        },
    }
}

pub fn write_single(session: Arc<RwLock<Session>>, node_id: &NodeId, value: Variant) -> bool {
    let session = session.write();
    let value = DataValue {
        value: Some(value),
        status: Some(StatusCode::Good),
        ..Default::default()
    };
    let write_value = WriteValue {
        node_id: node_id.clone(),
        attribute_id: AttributeId::Value as u32,
        index_range: UAString::null(),
        value,
    };
    let write_values = vec![write_value];
    let res = session.write(&write_values);
    match res {
        Ok(res) => {
            if res[0].is_good() {
                debug_println!("Write operation success");
                return true;
            }else{
                debug_println!("Write operation failed for bad or uncertain condition");
            }
        },
        Err(err) => {
            debug_println!("Write failed for {:?}",err);
        },
    }
    return false;
}

