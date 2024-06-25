use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug,sqlx::FromRow,Deserialize,Serialize,Default)]
pub struct TurbineState{
    pub id: i32,
    pub outlet_pressure: f64,
    pub pre_pressure: f64,
    pub frequency: f64,
    pub current: f64,
    pub safe_pressure: f64,
    pub power: bool,
    pub flow_rate: f64,
    pub flux: f64,
    pub open: f64,
}

impl TurbineState {
    pub fn new() -> TurbineState{
        let mut rng = rand::thread_rng();
        TurbineState{
            id: 0,
            outlet_pressure: rng.gen_range(0.0..50.0),
            pre_pressure: rng.gen_range(0.0..50.0),
            frequency: rng.gen_range(0.0..50.0),
            current: rng.gen_range(0.0..50.0),
            safe_pressure: rng.gen_range(0.0..50.0),
            power: true,
            flow_rate: rng.gen_range(0.0..50.0),
            flux: rng.gen_range(0.0..50.0),
            open: rng.gen_range(0.0..50.0),
        }
    }
}