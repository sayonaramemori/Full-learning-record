use std::sync::Arc;
use opcua::{client::prelude::{Client, Session,*}, sync::*};
use chrono::prelude::*;
use chrono::DateTime;
use std::fmt::Display;
use crate::debug_println;
use crate::utility::time::*;

#[derive(Clone,Debug)]
pub struct DataTime{pub data:String,pub time: DateTime<FixedOffset>}

impl Display for DataTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}|{:?}",self.data,self.time)
    }
}

pub struct OpcuaSession {
    session: Arc<std::sync::RwLock<Option<Arc<RwLock<Session>>>>>,
    url: String,
}

impl OpcuaSession {
    pub fn new(endpoint_url:&str) -> OpcuaSession {
        let res = OpcuaSession{
            session: Arc::new(std::sync::RwLock::new(None)),
            url: endpoint_url.to_string(),
        };
        res.gain_new_session();
        return res;
    }

    fn get_client()-> Client {
        ClientBuilder::new()
            .application_name("My First Client")
            .application_uri("urn:MyFirstClient")
            .create_sample_keypair(true)
            .trust_server_certs(true)
            .session_retry_limit(9)
            .session_timeout(999999999)
            .client().unwrap()
    }

    fn gain_new_session(&self){
        let mut session = self.session.write().unwrap();
        *session = Some(Self::get_client()
            .connect_to_endpoint(self.url.as_ref(), IdentityToken::Anonymous)
            .expect("connect failed"));
    }

    pub fn read_batch(&self, node_id: &Vec<NodeId>) -> Result<Vec<Option<DataTime>>,StatusCode>{
        let guard = self.session.read().unwrap();
        let session = guard.as_ref().unwrap().read();
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
                Ok(res)
            },
            Err(err) => {
                debug_println!("Read batch failed for {:?}",err);
                drop(session);
                drop(guard);
                self.gain_new_session();
                return Err(err);
            },
        }
    }

    pub fn read_single(&self, node_id: &NodeId) -> Result<Option<DataTime>,StatusCode>{
        let guard = self.session.read().unwrap();
        let session = guard.as_ref().unwrap().read();
        let temp :Vec<ReadValueId>= vec![node_id.into()];
        match session.read(&temp, TimestampsToReturn::Both, 0.0){
            Ok(res) => {
                for i in res {
                    let time = i.server_timestamp.unwrap().as_chrono();
                    let time = with_timezone(time);
                    if let Some(val) = i.value {
                        return Ok(Some(DataTime{data:val.to_string(),time}));
                    }
                }
                return Ok(None);
            },
            Err(err) => {
                drop(session);
                drop(guard);
                self.gain_new_session();
                Err(err)
            },
        }
    }
    pub fn write_single_retry(&self, node_id: &NodeId, value: Variant, times:u32)->bool{
        for _ in 0..times {
            if self.write_single(node_id, value.clone()) {
                return true;
            }
        }
        false
    }
    pub fn write_single(&self, node_id: &NodeId, value: Variant) -> bool {
        let guard = self.session.read().unwrap();
        let session = guard.as_ref().unwrap().read();
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
        if let Ok(res) = res {
            if res[0].is_good(){
                debug_println!("Write operation success");
                return true;
            }
        }
        debug_println!("Write failed");
        drop(session);
        drop(guard);
        self.gain_new_session();
        return false;
    }
}











