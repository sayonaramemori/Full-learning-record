use chrono::prelude::*;
pub struct Student {
    pub id: u64,
    pub name: String,
    pub age: u16,
    pub id_card: String,
    pub last_changed_on: NaiveDate,
}

impl Student {
    pub fn new(namet:String,aget:u16,id_cardt:String)->Student{
        return Student{
            id: 1,
            name: namet,
            age: aget,
            id_card: id_cardt,
            last_changed_on: chrono::NaiveDate::from_ymd(2100,3,3),
        }

    }
}