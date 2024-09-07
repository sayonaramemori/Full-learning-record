use tokio::time::sleep;
use tokio::sync::broadcast::{self,Sender,Receiver};
use mysql::*;
use std::sync::Arc;
use crate::debug_println;
use crate::opcua_config::opcua_session_wrapper::OpcuaSession;
use super::interface::collect::StoreValueTime;

pub struct DataCollector<T> {
    target: String,
    sender: Arc<Sender<T>>,
    collect_interval: usize,
}

impl<T> DataCollector<T> 
where T: 'static + Clone + Send + Sync + StoreValueTime
{
    pub fn new(target:&str,pipe_size: usize) -> DataCollector<T> {
        let (sender, _rx) = broadcast::channel::<T>(pipe_size);
        let res = DataCollector { 
            target: target.to_string(), 
            sender: Arc::new(sender),
            collect_interval: 1,
        };
        return res;
    }

    pub async fn execute(&self) -> Result<(),String>{
        let session= OpcuaSession::new_arc().await;
        match OpcuaSession::async_read(session,self.target.as_ref()).await {
            Ok(res) => { let _ = self.sender.send(res); },
            Err(e) => { return Err(format!("Read node failed for {e}")); },
        }
        Ok(())
    }

    //Should be called after subscription
    pub async fn execute_loop(collector: DataCollector<T>) -> Result<(),String>{
        collector.start_with_one_target().await;
        Ok(())
    }

    async fn start_with_one_target(&self){
        let session= OpcuaSession::new_arc().await;
        loop {
            let session_clone = session.clone();
            match OpcuaSession::async_read(session_clone,self.target.as_ref()).await {
                Ok(res) => {
                    let _ = self.sender.send(res);
                },
                Err(_e) => { debug_println!("Reading node failed"); },
            };
            sleep(std::time::Duration::from_secs(self.collect_interval as u64)).await;
        }
    }
    pub fn subscribe(&self) -> Receiver<T>{
        self.sender.subscribe()
    }
}

