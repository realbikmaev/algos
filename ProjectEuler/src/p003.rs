use crate::utils::prime_factors_of;

pub fn problem_003(n: i64) -> i64 {
    let res = prime_factors_of(n).iter().max().to_owned().unwrap().clone();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_003() {
        assert_eq!(problem_003(13195), 29);
    }
    #[test]
    fn verify_003() {
        assert_eq!(problem_003(600851475143), 6857);
    }
}
