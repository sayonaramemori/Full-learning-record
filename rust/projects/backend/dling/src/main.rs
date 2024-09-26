struct MyMatrix<T:Clone+Default>{
   matrix: Vec<Vec<T>>,
}

impl<T:Clone + Default> MyMatrix<T> {
    pub fn zeros(init_val: T,col:usize,row:usize)->MyMatrix<T>{
        let row_temp = vec![init_val;col];
        let matrix = vec![row_temp;row];
        MyMatrix { matrix }
    }
}

fn relu(x:f64) -> f64 {
    if x>0.0 {x} else {0.0}
}


fn main() {
    println!("Hello, world!");
}
