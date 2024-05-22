extern crate mysql_test;
use mysql::*;
use mysql::prelude::*;
use mysql_test::stu::student::Student;
use mysql_test::*;
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let url= env::var("DATABASE_URL").expect("ENV not found");
    let opt = Opts::from_url(&url).unwrap();
    let pool= Pool::new(opt).unwrap();
    let mut conn = pool.get_conn().unwrap();
    testQuery(conn.as_mut());
    let stus:Vec<Student> = vec![
        Student::new("Rust".to_string(),88,"hhhhhhh".to_string()),
        Student::new("Python".to_string(), 99, "greatest".to_string()),
    ];
    // testInsert(conn.as_mut(), &stus);
    println!("Hello, world!");
}