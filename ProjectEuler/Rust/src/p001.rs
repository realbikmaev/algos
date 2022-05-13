pub fn problem_001(below: i32) -> i32 {
    let res = (1..below).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    // print!("{res}");
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_001() {
        assert_eq!(problem_001(1000), 233168);
    }
    #[test]
    fn verify_001() {
        assert_eq!(problem_001(10), 23);
    }
}
