use mysql::prelude::*;
use mysql::*;
extern crate AutoReagent;
use AutoReagent::models::TurbineState::TurbineState;
use std::thread;
use std::time::Duration as StdDuration;

fn main() {
    let url = "mysql://root:121234@kazusa.vip:3000/plc";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let statement = r"UPDATE turbineState 
        SET outlet_pressure = :outlet_pressure, 
            pre_pressure = :pre_pressure ,
            frequency = :frequency,
            current = :current,
            safe_pressure = :safe_pressure,
            power = :power,
            flow_rate = :flow_rate,
            flux = :flux,
            open = :open
        WHERE id = ";
    let s1 = statement.to_string() + "1";
    let s2 = statement.to_string() + "2";
    loop {
        let temp1 = TurbineState::new(true);
        let para1 = params! {
                "outlet_pressure" => temp1.outlet_pressure,
                "pre_pressure" => temp1.pre_pressure,
                "frequency" => temp1.frequency,
                "current" => temp1.current,
                "safe_pressure" => temp1.safe_pressure,
                "power" => temp1.power,
                "flow_rate" => temp1.flow_rate,
                "flux" => temp1.flux,
                "open" => temp1.open,
            };
        let temp2 = TurbineState::new(true);
        let para2 = params! {
                "outlet_pressure" => temp2.outlet_pressure,
                "pre_pressure" => temp2.pre_pressure,
                "frequency" => temp2.frequency,
                "current" => temp2.current,
                "safe_pressure" => temp2.safe_pressure,
                "power" => temp2.power,
                "flow_rate" => temp2.flow_rate,
                "flux" => temp2.flux,
                "open" => temp2.open,
            };
        conn.exec_drop(&s1,para1).unwrap();
        conn.exec_drop(&s2,para2).unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
    }
}