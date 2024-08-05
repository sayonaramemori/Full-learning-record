use std::sync;
use mysql::*;

impl MysqlData {
    pub fn new(url:&str) -> MysqlData {
        let pool = Pool::new(url).unwrap();
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