use crate::opcua_config::opcua_session_wrapper::OpcuaSession;
use super::interface::transfer::InstructionInfo;

pub struct DataTransferee;
impl DataTransferee {
    pub async fn execute<T>(ins:&T) -> bool 
    where T: InstructionInfo
    {
        match OpcuaSession::async_write_once(&ins.get_target(),ins.get_value()).await{
            Ok(()) => { true },
            Err(_e) => { false }
        }
    }
}

