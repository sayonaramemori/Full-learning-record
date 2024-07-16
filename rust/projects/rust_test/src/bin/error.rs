use std::prelude::*;

use std::fmt::Error;


fn main() {
    println!("hello world");
}

fn result(a:i32) -> Result<(),Error> {
    if a<0 {
        return Ok(());
    }else {
        return Err(Error);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn te() {
        let res = result(1);

        assert_eq!(Ok(()),result(-1));
    }
}

