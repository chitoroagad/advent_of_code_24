fn main() {
    println!("hello world")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(true, true)
    }
}
