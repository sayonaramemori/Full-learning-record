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
    pub fn new_variant_mapper() -> Self {
        let res: DataStore = DataStore { data: Arc::new(RwLock::new(HashMap::new())), };
        res.insert_func(|val:bool|{return Variant::Boolean(val);});
        res.insert_func(|val:f32|{return Variant::Float(val);});
        res.insert_func(|val:f64|{return Variant::Double(val);});
        res.insert_func(|val:i32|{return Variant::Int32(val);});
        res.insert_func(|val:i64|{return Variant::Int64(val);});
        res.insert_func(|val:i16|{return Variant::Int16(val);});
        res.insert_func(|val:u16|{return Variant::UInt16(val);});
        res.insert_func(|val:u32|{return Variant::UInt32(val);});
        res.insert_func(|val:u64|{return Variant::UInt64(val);});
        return res;
    }

    pub fn get_func<T>(&self) -> Option<Arc<fn(T)->Variant>>
    where T: 'static + Send + Sync ,
    {
        let data = self.data.read().unwrap();
        let res = data.get(&TypeId::of::<T>());
        if let Some(res) = res {
            let res = res.clone().downcast::<fn(T)->Variant>();
            if let Ok(res) = res {
                return Some(res);
            }
        }
        None
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
