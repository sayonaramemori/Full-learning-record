use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
     dotenvy::dotenv()?;
     println!("{}",dotenvy::var("java").unwrap());
     Ok(())
}
