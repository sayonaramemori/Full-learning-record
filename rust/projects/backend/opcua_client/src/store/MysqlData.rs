use std::sync;
use mysql::*;

impl MysqlData {
    pub async fn new(url:&'static str) -> MysqlData {
        let pool = tokio::task::spawn_blocking(move ||{Pool::new(url).unwrap()}).await.unwrap();
        MysqlData {pool:sync::RwLock::new(pool)}
    }
    pub fn get_conn(&self) -> Result<PooledConn,mysql::Error> {
        let guard = self.pool.write().unwrap();
        guard.get_conn()
    }
}

pub struct MysqlData{
    pool: sync::RwLock<Pool>,
}