### Create a new project  
```rust  
cargo new mysql-test-01
[dependencies]
mysql="*"
chrono="0.4"
```

### Geting Started  
```rust
use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;

fn main() {
    let url = "mysql://root:password@ayanamyrei.com:3306/mydb";
    let pool = Pool::new(url).unwarp();
    let mut conn = pool.get_conn().unwarp();
}
```


### Stream query  
```rust  
conn.query_iter("select * from student")
    .unwarp()
    .for_each(|row| {
        let r:(i32,String,i32,String,NaiveDate) = from_row(row.unwarp());
        println!("{},{},{},{},{:?}",r.0,r.1,r.2,r.3,r.4);
    });
```

### Cluster query  
```rust
let res: Vec<(i32, String, i32, String, NaiveDate)> =
    conn.query("select * from student").unwrap();
    for r in res {
            println!("{}, {},{},{}, {:?}", r.0, r.1, r.2, r.3, r.4);
    }
```

### Result to structure  
```rust  
struct Student {
        id: u64,
        name: String,
        age: u16,
        id_card: String,
        last_changed_on: NaiveDate,
}

let res = conn.query_map(
    "select * from student",
    |(id, name, age, id_card, update)| Student {
        id: id,
        name: name,
        age: age,
        id_card: id_card,
        last_changed_on: update,
    },
).expect("Query failed.");

for i in res {
    println!(
        "{}, {},{},{}, {:?}",
        i.id, i.name, i.age, i.id_card, i.last_changed_on
    )
}
```

### Query single record  
```rust  
// 条件查询，查询单个数据
let res = conn.query_first("select * from student where name = '张三'")
.map(
    // Unpack Result
    |row| {
    row.map(|(id, name, age, id_card, update)| Student {
            id: id,
            name: name,
            age: age,
            id_card: id_card,
            last_changed_on: update,
        })
    },
);

match res.unwrap() {
    Some(student) => println!(
        "{}, {},{},{}, {:?}",
        student.id, student.name, student.age, student.id_card, student.last_changed_on
    ),
    None => println!("Sorry no student found."),
}
```

