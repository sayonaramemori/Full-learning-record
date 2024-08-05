#![feature(stmt_expr_attributes)]

pub mod store;
pub mod Models;
pub mod Example;
pub mod opcua_config;
pub mod utility;


#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            use chrono::prelude::*;
            print!("{}   ",chrono::Local::now().with_nanosecond(0).unwrap());
            println!($($arg)*);
        }
    }
}