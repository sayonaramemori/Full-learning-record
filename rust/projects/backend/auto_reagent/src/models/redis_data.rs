use std::sync::Arc;
use redis::{aio::MultiplexedConnection, AsyncCommands, FromRedisValue, RedisError, RedisResult};
#[derive(Clone)]
pub struct RedisState {
    pub redis_client: Arc<redis::Client>,
    pub redis_passwd: String,
}

impl RedisState {
    pub fn new(pass:&str,client: redis::Client) -> RedisState{
        return RedisState{redis_client: Arc::new(client),redis_passwd:pass.to_string()}
    }
    pub async fn get_auth_connection(&self) ->Result<MultiplexedConnection, redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        redis::cmd("AUTH").arg(&self.redis_passwd).query_async(&mut conn).await?;
        Ok(conn)
    }
    pub async fn set(&self,key:&str,val:String) ->RedisResult<()> {
        let mut conn = self.get_auth_connection().await?;
        match redis::cmd("SET").arg(key).arg(val).query_async::<_,()>(&mut conn).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    pub async fn setex(&self,key:&str,val:String,sec:u32) ->RedisResult<()> {
        let mut conn = self.get_auth_connection().await?;
        match redis::cmd("SETEX").arg(key).arg(sec.to_string()).arg(val).query_async::<_,()>(&mut conn).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    pub async fn get(&self,key:&str) ->RedisResult<String>{
        let mut conn = self.get_auth_connection().await?;
        match redis::cmd("GET").arg(key).query_async::<_,String>(&mut conn).await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
    pub async fn lpush_ex(&self,key:&str,args:Vec<String>,sec:u32) ->RedisResult<()>{
        let mut conn = self.get_auth_connection().await?;
        let mut temp = redis::pipe();
        temp.add_command(redis::cmd("LPUSH")).arg(key);
        args.into_iter().map(|arg|{temp.arg(arg);}).last();
        temp.add_command(redis::cmd("EXPIRE")).arg(key).arg(sec.to_string());
        match temp.query_async::<_,()>(&mut conn).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    pub async fn lrange(&self,key:&str,count:usize) -> RedisResult<Vec<String>> {
        let mut conn = self.get_auth_connection().await?;
        match redis::cmd("LRANGE").arg(key).arg("-".to_string() + &count.to_string()).arg("-1").query_async::<_,Vec<String>>(&mut conn).await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
    pub async fn hgetall<T>(&self,keys:Vec<&str>)  -> RedisResult<Vec<T>> 
    where T: FromRedisValue
    {
        let mut conn = self.get_auth_connection().await?;
        let mut pipe = redis::pipe();
        let cmd = redis::cmd("HGETALL");
        for key in keys {pipe.add_command(cmd.clone()).arg(key);}
        match pipe.query_async::<_,Vec<T>>(&mut conn).await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
}