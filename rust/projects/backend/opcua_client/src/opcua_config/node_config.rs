use std::str::FromStr;
use opcua::types::NodeId;
use serde::Deserialize;
use super::dataType::DataType;
use opcua::types::Variant;

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
}

impl NodeConfig {
    pub async fn new() -> NodeConfig{
        let res = tokio::task::spawn_blocking(move||{
            let content = std::fs::read_to_string("C:\\Users\\13427\\Desktop\\code\\linux-tools\\rust\\projects\\backend\\opcua_client\\src\\opcua_config\\config.yml").unwrap();
            serde_yml::from_str::<NodeConfig>(&content).unwrap()
        }).await.unwrap();
        return res;
    }
    pub fn get_type(&self,tag:&str) -> Option<DataType> {
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
    pub fn get_node(&self,tag:&str) -> &str{
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
    //if no matched node in config.yml, error will be raised directly
    pub fn node(&self,tag:&str) -> NodeId {
        NodeId::from_str(self.get_node(tag)).unwrap()
    }
    pub fn get_variant(&self,tag:&str,val:String) -> Option<Variant>{
        let dt = Self::get_type(self, tag).unwrap();
        match dt {
            DataType::Bool => {
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
        None
    }
}