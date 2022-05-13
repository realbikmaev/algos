fn prime_factors(n: i64) -> Vec<i64> {
    let mut n = n;
    let mut res = Vec::new();
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    if n > 1 {
        res.push(n);
    }
    res
}

pub fn problem_003(n: i64) -> i64 {
    let res = prime_factors(n).iter().max().to_owned().unwrap().clone();
    // print!("{res}");
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
