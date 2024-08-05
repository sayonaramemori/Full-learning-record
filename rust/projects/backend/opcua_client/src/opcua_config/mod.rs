use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use opcua::types::NodeId;
use serde::Deserialize;
pub mod Operation;

#[derive(Deserialize, Debug, Clone)]
pub struct Mapping {
    tag: String,
    node: String,
}

#[derive(Deserialize, Debug)]
pub struct NodeConfig{
    produce: Option<Vec<Mapping>>,
    test: Option<Vec<Mapping>>,
}

impl NodeConfig {
    pub fn new() -> NodeConfig{
        let mut file = File::open("C:\\Users\\13427\\Desktop\\code\\linux-tools\\rust\\projects\\backend\\opcua_client\\src\\opcua_config\\config.yml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_yml::from_str::<NodeConfig>(&contents).unwrap()
    }
    pub fn get(&self,tag:&str) -> &str{
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
        NodeId::from_str(self.get(tag)).unwrap()
    }
}