pub fn add_one(input: i32) -> i32 {
    input + 1
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, add_one(1));
    }
}