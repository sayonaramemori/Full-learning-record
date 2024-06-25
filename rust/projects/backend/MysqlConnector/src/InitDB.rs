use mysql::*;
use mysql::prelude::*;


#[derive(Debug)]
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
}

pub struct MysqlPool {
    // pub pool: Mutext<Pool>,
}

fn get_conn(){

}

pub fn test() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:121234@kazusa.vip:3000/plc";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let payments = vec![
        TurbineState {
            id: 1,
            outlet_pressure: 23.3,
            pre_pressure: 23.3,
            frequency: 50.0,
            current: 1.0,
            safe_pressure: 100.0,
            power: true,
            flow_rate: 23.33,
            flux: 1000.0,
        },
        TurbineState {
            id: 2,
            outlet_pressure: 23.3,
            pre_pressure: 23.3,
            frequency: 50.0,
            current: 1.0,
            safe_pressure: 100.0,
            power: true,
            flow_rate: 23.33,
            flux: 1000.0,
        }
    ];

    // Now let's insert payments to the database
    // conn.exec_batch(
    //     r"INSERT INTO turbineState(id, outlet_pressure, pre_pressure, frequency, current, safe_pressure, power, flow_rate, flux)
    //       VALUES (:id, :outlet_pressure, :pre_pressure, :frequency, :current, :safe_pressure, :power, :flow_rate, :flux)",
    //     payments.into_iter().map(|p| params! {
    //         "id" => p.id,
    //         "outlet_pressure" => p.outlet_pressure,
    //         "pre_pressure" => p.pre_pressure,
    //         "frequency" => p.frequency,
    //         "current" => p.current,
    //         "safe_pressure" => p.safe_pressure,
    //         "power" => p.power,
    //         "flow_rate" => p.flow_rate,
    //         "flux" => p.flux,
    //     })
    // )?;

    // Let's select payments from database. Type inference should do the trick here.
    let selected_payments = conn
        .query_map(
            "SELECT id, outlet_pressure, pre_pressure, frequency, current, safe_pressure, power, flow_rate, flux from turbineState",
            |(id, outlet_pressure, pre_pressure, frequency, current, safe_pressure, power, flow_rate, flux)| {
                TurbineState{id, outlet_pressure, pre_pressure, frequency, current, safe_pressure, power, flow_rate, flux}
            },
        )?;

    println!("{:?}",selected_payments);
    println!("Yay!");

    Ok(())
}