use std::fmt::Debug;
use std::ops::Add;
use std::ops::Mul;
use std::fmt::Display;

#[derive(Debug,Clone)]
struct MyMatrix<T>
where T: Clone + Copy + Default + Add + Debug
{
   matrix: Vec<Vec<T>>,
}

impl<T> Mul<Self> for MyMatrix<T>
where T: Clone + Copy + Default + Add<Output = T> + Debug + Mul<Output = T>
{
    type Output = Option<MyMatrix<T>>;
    fn mul(self, mut rhs: Self) -> Self::Output {
        if self.col() != rhs.row() { return None; }
        let mut res = MyMatrix::new(T::default(), self.row(), rhs.col());
        let rhs = rhs.transpose();
        for r in 0..res.row() {
            for c in 0..res.col() {
                let mut temp = T::default();
                for i in 0..self.matrix[r].len() {
                    temp = temp + self.matrix[r][i] * rhs.matrix[r][i];
                }
                res.matrix[r][c] = temp;
            }
        }
        Some(res)
    }
}

impl<T> Display for MyMatrix<T>
where T: Clone + Copy + Default + Add + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.matrix.iter() {
            writeln!(f,"{:?}",row)?
        }
        Ok(())
    }
}

impl<T> MyMatrix<T>
where T: Clone + Copy + Default + Add + Debug
{
    pub fn row(&self) -> usize {
        self.matrix.len()
    }
    pub fn col(&self) -> usize {
        self.matrix.first().map(|v| v.len()).unwrap_or(0)
    }
    pub fn fn_new<F>(init_val: F,row:usize, col:usize,)-> MyMatrix<T>
    where F: Fn() -> T
    {
        let row_temp = vec![init_val();col];
        let matrix = vec![row_temp;row];
        MyMatrix { matrix }
    }
    pub fn new(init_val: T,row:usize, col:usize,)-> MyMatrix<T>{
        let row_temp = vec![init_val;col];
        let matrix = vec![row_temp;row];
        MyMatrix { matrix }
    }
    pub fn transpose(&mut self) -> MyMatrix<T> {
        let row = self.matrix.len();
        let col = self.matrix.first().map(|v| v.len()).unwrap_or(0);
        let mut new_matrix = Self::new(T::default(), col, row);
        for c in 0..col {
            for r in 0..row {
                new_matrix.matrix[c][r] = self.matrix[r][c];
            }
        }
        new_matrix
    }
    pub fn _assign(&mut self, row:usize, col: usize, val:T) -> Option<&MyMatrix<T>> {
        match self.matrix.get_mut(row) {
            Some(row) => {
                match row.get_mut(col) {
                    Some(val_indexd) => { *val_indexd = val },
                    None => return None,
                }
            },
            None => return None,
        }
        None
    }
}

fn relu(x:f64) -> f64 {
    if x>0.0 {x} else {0.0}
}

fn main() {
    let mut m1 = MyMatrix::fn_new(|| f64::sin(9.9), 5, 8);
    let mut m2 = MyMatrix::new(9.8, 8, 5);
    println!("{}",m1);
    println!("{}",m2);
    println!("{}",(m1*m2).unwrap());
    println!("Hello, world!");
}
