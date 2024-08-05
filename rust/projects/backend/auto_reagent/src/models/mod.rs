pub mod TempRecord;
pub mod LoginInfo;
pub mod TurbineState;
pub mod redis_data;

pub enum Operation {
    Start,
    Stop,
}

impl Operation {
    pub fn to_number(&self) -> usize {
        match self {
            Operation::Start => 1,
            Operation::Stop => 0,
        }
    }
    pub fn to_bool(&self) -> bool{
        match self {
            Operation::Start => true,
            Operation::Stop => false,
        }
    }
}