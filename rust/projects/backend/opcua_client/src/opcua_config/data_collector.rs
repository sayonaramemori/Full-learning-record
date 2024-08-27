use opcua::types::StatusCode;
use tokio::time::sleep;
use tokio::sync::broadcast::{self,Sender,Receiver};
use mysql::*;
use std::sync::Arc;
use crate::debug_println;
use crate::opcua_config::opcua_session_wrapper::{DataTime,OpcuaSession};

pub struct DataCollector<T> {
    target: String,
    sender: Arc<Sender<T>>,
    collect_interval: usize,
    callback_ok: Option<Box<dyn Fn(&T)->()>>,
    callback_err: Option<Box<dyn Fn(StatusCode)->()>>,
}

impl<T> DataCollector<T> 
where T: 'static + Clone + Send + Sync + From<DataTime> 
{
    pub fn new(target:&str,pipe_size: usize) -> DataCollector<T> {
        let (sender, _rx) = broadcast::channel::<T>(pipe_size);
        let res = DataCollector { 
            target: target.to_string(), 
            sender: Arc::new(sender),
            collect_interval: 1,
            callback_ok: None,
            callback_err: None,
        };
        return res;
    }

    //Should be called after subscription
    pub async fn start(collector: DataCollector<T>) -> Result<(),Box<dyn std::error::Error>>{
        collector.start_with_one_target().await;
        Ok(())
    }

    async fn start_with_one_target(&self){
        let session= OpcuaSession::new_arc().await;
        loop {
            let session_clone = session.clone();
            match OpcuaSession::async_read(session_clone,self.target.as_ref()).await {
                Ok(res) => {
                    if let Some(func) = self.callback_ok.as_ref(){ func(&res); }
                    let _ = self.sender.send(res);
                },
                Err(e) => {
                    if let Some(func) = self.callback_err.as_ref(){ func(e); }
                    debug_println!("Reading node failed");
                },
            };
            sleep(std::time::Duration::from_secs(self.collect_interval as u64)).await;
        }
    }
    pub fn subscribe(&self) -> Receiver<T>{
        self.sender.subscribe()
    }
    pub fn set_success_callback<F>(&mut self,f: F)
    where F: Fn(&T)->() + 'static
    {
        self.callback_ok = Some(Box::new(f));
    }
    pub fn set_fail_callback<F>(&mut self,f: F)
    where F: Fn(StatusCode)->() + 'static
    {
        self.callback_err = Some(Box::new(f));
    }
}

