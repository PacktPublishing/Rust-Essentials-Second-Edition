#[cfg(not(test))]
fn main() {
    println!("Normal mode, no test was compiled");
}

pub fn double(n: i32) -> i32 {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(double(42), 84);
    }
}
