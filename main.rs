fn main() {
    println!("Hello, world!");
    println!("{}",sum(2,3));
}

pub fn sum(x: i64, y: i64) -> i64{
    x + y
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(1, 2), 3);
    }

}