use crate::utils::is_divisible_by_all;

pub fn problem_005(below: i64) -> i64 {
    let primes: Vec<i64> = (2..below).collect();
    let mut n: i64 = 0;
    loop {
        n += 1;
        if is_divisible_by_all(n, &primes) {
            break;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_005() {
        assert_eq!(problem_005(10), 2520);
    }
    #[test]
    fn verify_005() {
        assert_eq!(problem_005(20), 232792560);
    }
}
