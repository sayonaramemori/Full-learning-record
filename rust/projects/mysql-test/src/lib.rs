use mysql::*;
use mysql::prelude::*;

pub mod stu;
use stu::student::Student;

pub fn testQuery(conn:&mut Conn) {
    let res=conn.query_map(
        "select * from student",
        |(id, name, age, id_card, update)| Student {
                id: id,
                name: name,
                age: age,
                id_card: id_card,
                last_changed_on: update,
            },
    ).expect("Query faild");
    for i in res {
        println!(
            "|{}|{}|{}|{}|{:?}|",
            i.id, i.name, i.age, i.id_card, i.last_changed_on
        )
    }
}

pub fn testInsert(conn:&mut Conn,stus:&Vec<Student>) {
    conn.exec_batch("insert into student (name, age, id_card, last_update) values (:name, :age, :phone, CURRENT_DATE());",
    stus.iter().map(|p| params! {
        "name" => p.name.clone(),
        "age" => p.age,
        "phone" => p.id_card.clone(),
    }));
}






