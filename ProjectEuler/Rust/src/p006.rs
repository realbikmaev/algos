pub fn problem_006(n: i64) -> i64 {
    let sum_of_squares = (1..=n).map(|x| x.pow(2)).sum::<i64>();
    let square_of_sum = (1..=n).sum::<i64>().pow(2);
    let res = square_of_sum - sum_of_squares;
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_006() {
        assert_eq!(problem_006(10), 2640);
    }
    #[test]
    fn verify_006() {
        assert_eq!(problem_006(100), 25164150);
    }
}
