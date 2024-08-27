use std::{collections::HashMap, str::FromStr};
use opcua::types::NodeId;
use serde::Deserialize;
use super::data_type::DataType;
use opcua::types::Variant;
use tokio::sync::OnceCell as TokioOnceCell;
use std::sync::Arc;

static CONFIG: TokioOnceCell<Arc<NodeConfig>> = TokioOnceCell::const_new();
pub async fn get_node_config() -> Arc<NodeConfig> {
    CONFIG.get_or_init(|| async {
        Arc::new(NodeConfig::new().await)
    }).await.clone()
}

#[derive(Deserialize, Debug, Clone)]
pub struct Mapping {
    tag: String,
    node: String,
    dtype: Option<DataType>,
}


#[derive(Deserialize, Debug)]
pub struct NodeConfig{
    produce: Option<Vec<Mapping>>,
    test: Option<Vec<Mapping>>,
    node_built: Option<HashMap<String,NodeId>>
}

impl NodeConfig {
    pub async fn new() -> NodeConfig{
        //This unwrap only happens at init stage, so it is safe
        let mut res = tokio::task::spawn_blocking(move||{
            let content = std::fs::read_to_string("C:\\Users\\13427\\Desktop\\code\\linux-tools\\rust\\projects\\backend\\opcua_client\\src\\opcua_config\\config.yml").unwrap();
            serde_yml::from_str::<NodeConfig>(&content).unwrap()
        }).await.unwrap();
        res.init_node_store();
        return res;
    }

    //Init node_built
    fn init_node_store(&mut self){
        self.node_built = Some(HashMap::new());
        if let Some(ref produce) = self.produce {
            let empty :Vec<Mapping>= vec![];
            let mut iters = produce.into_iter().chain(empty.iter());
            if let Some(ref test) = self.test { iters = produce.into_iter().chain(test.into_iter()); }
            iters.map(|val|{
                //This unwrap check the config syntax
                let id = NodeId::from_str(&val.node).unwrap();
                self.node_built.as_mut().unwrap().insert(val.tag.clone(),id);
            }).last();
        }
    }
    
    fn get_type(&self,tag:&str) -> Option<DataType> {
         if let Some(ref val) = self.produce {
            for i in val {
                if i.tag == tag {
                    return i.dtype.clone();
                }
            }
        }
        if let Some(ref val) = self.test {
            for i in val {
                if i.tag == tag {
                    return i.dtype.clone();
                }
            }
        }
        None
    }

    //description for a node 
    fn get_node_str(&self,tag:&str) -> &str{
        if let Some(ref val) = self.produce {
            for i in val {
                if i.tag == tag {
                    return &i.node;
                }
            }
        }
        if let Some(ref val) = self.test {
            for i in val {
                if i.tag == tag {
                    return &i.node;
                }
            }
        }
        ""
    }

    //this unwrap is safe due to it has been initialized
    pub fn node(&self,tag:&str) -> Option<NodeId> {
        let res = self.node_built.as_ref().unwrap().get(tag);
        match res {
            Some(node) => return Some(node.clone()),
            _ => None,
        }
    }

    //Get a variant with the same type of specific tag, set value
    pub fn get_variant(&self,tag:&str,val:String) -> Option<Variant>{
        if let Some(dt) = Self::get_type(self, tag){
            match dt {
                DataType::Boolean => {
                    if let Ok(val)= val.parse::<bool>(){return Some(Variant::Boolean(val))}
                },
                DataType::Double => {
                    if let Ok(val)= val.parse::<f64>(){  return Some(Variant::Double(val))}
                },
                DataType::Float => {
                    if let Ok(val)= val.parse::<f32>(){  return Some(Variant::Float(val))}
                },
                DataType::Int16 => {
                    if let Ok(val)= val.parse::<i16>(){  return Some(Variant::Int16(val))}
                },
                DataType::Int32 => {
                    if let Ok(val)= val.parse::<i32>(){  return Some(Variant::Int32(val))}
                },
                DataType::Int64 => {
                    if let Ok(val)= val.parse::<i64>(){  return Some(Variant::Int64(val))}
                },
                DataType::UInt16 => {
                    if let Ok(val)= val.parse::<u16>(){  return Some(Variant::UInt16(val))}
                },
                DataType::UInt32 => {
                    if let Ok(val)= val.parse::<u32>(){  return Some(Variant::UInt32(val))}
                },
                DataType::UInt64 => {
                    if let Ok(val)= val.parse::<u64>(){  return Some(Variant::UInt64(val))}
                },
            };
        }
        None
    }
}



