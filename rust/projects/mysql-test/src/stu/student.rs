use chrono::prelude::*;

#[derive(Clone)]
pub struct Student {
    pub id: u64,
    pub name: String,
    pub age: u16,
    pub id_card: String,
    pub last_changed_on: NaiveDate,
}

impl Student {
    pub fn new(name:&str)->Student{
        return Student{
            id: 1,
            name: name.to_string(),
            age: 99,
            id_card: "id_card".to_string(),
            last_changed_on: chrono::NaiveDate::from_ymd(2100,3,3),
        }
    }
}
