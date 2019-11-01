#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

}


pub fn add_two(i: i32) -> i32 {
    i+2
}
