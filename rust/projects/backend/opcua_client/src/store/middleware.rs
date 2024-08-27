use std::any::{Any, TypeId};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::SeekFrom;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use opcua::types::Variant;

pub struct DataStore {
    data: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore { data: Arc::new(RwLock::new(HashMap::new())), }
    }

    fn insert_func<T>(&self, f:fn(T)->Variant)
    where T: 'static + Send + Sync,
    {
        let mut data = self.data.write().unwrap();
        data.insert(TypeId::of::<T>(), Arc::new(f) as Arc<dyn Any + Send + Sync>);
    }

    pub fn insert<T>(&self, value: T)
    where T: 'static + Send + Sync 
    {
        let mut data = self.data.write().unwrap();
        data.insert(TypeId::of::<T>(), Arc::new(value));
    }
    
    //this function has potential bug when use at async environment
    pub fn insert_arc<T>(&self, value: Arc<T>)
    where T: 'static + Send + Sync 
    {
        let mut data = self.data.write().unwrap();
        data.insert(TypeId::of::<T>(), value.clone());
    }
    
    pub fn get<T>(&self) -> Option<Arc<T>> 
    where T: 'static + Send + Sync 
    {
        let data = self.data.read().unwrap();
        let res = data.get(&TypeId::of::<T>());
        if let Some(res) = res {
            let res = res.clone().downcast::<T>();
            if let Ok(res) = res {
                return Some(res);
            }
        }
        None
    }
    
}
