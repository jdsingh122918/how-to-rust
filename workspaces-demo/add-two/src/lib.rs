pub fn add_two(input1:i32, input2: i32) -> i32 {
    input1 + input2
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn add_two_number() {
        assert_eq!(5, add_two(3, 2));
    }
}