use std::collections::HashMap;
use jwt::Store;
use sqlx::{mysql::MySqlPoolOptions,MySqlPool};

#[derive(Clone,Debug,Default)]
pub struct SqlxManager {
    databases: HashMap<String,MySqlPool>
}
impl SqlxManager {
    pub fn new() -> SqlxManager {
        SqlxManager { databases: HashMap::new() }
    }
    pub async fn add_database(&mut self,name:&str,url: &str){
        let pool = MySqlPoolOptions::new().connect(url).await.unwrap();
        self.databases.insert(name.to_string(), pool);
    }
    pub fn get(&self,db_name:&str) -> Option<&MySqlPool>{
        self.databases.get(db_name)
    }
}