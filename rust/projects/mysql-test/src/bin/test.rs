extern crate mysql_test;
use mysql::*;
use mysql::prelude::*;
use mysql_test::stu::student::Student;
use mysql_test::*;
use std::env;
use dotenv::dotenv;

const QUERYALL:&str = "select * from student";


fn main() {
    dotenv().ok();
    let url= env::var("DATABASE_URL").expect("ENV not found");
    let opt = Opts::from_url(&url).unwrap();
    let pool= Pool::new(opt).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let stus:Vec<Student> = vec![
        Student::new("javascripts"),
        Student::new("ruby"),
    ];
    //insert(conn.as_mut(), &stus[0]);
    query(conn.as_mut(),QUERYALL);
    println!("Hello, world!");
}
