use crate::utils::fib_iter;

pub fn problem_002(below: i64) -> i64 {
    let res = fib_iter(below).filter(|x| x % 2 == 0).sum();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_002() {
        assert_eq!(problem_002(50), 44);
    }
    #[test]
    fn verify_002() {
        assert_eq!(problem_002(4_000_000), 4613732);
    }
}
