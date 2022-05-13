fn erathostenes_sieve_iter(n: i64) -> i64 {
    let huge_n = (n.pow(2) + 1) as usize;
    let mut sieve = vec![true; huge_n];
    sieve[0] = false;
    sieve[1] = false;
    sieve[2] = true;
    let mut n_of_prime = 1_i64;
    let mut prime = 2_i64;
    loop {
        if sieve[prime as usize] {
            if n_of_prime == n {
                break;
            }
            n_of_prime += 1;
            for j in (prime * prime..huge_n.try_into().unwrap()).step_by(prime as usize) {
                sieve[j as usize] = false;
            }
        }
        prime += 1;
    }
    prime
}

pub fn problem_007(n: i64) -> i64 {
    let res = erathostenes_sieve_iter(n);
    // print!("{res}");
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_007() {
        assert_eq!(problem_007(6), 13);
    }
    #[test]
    fn verify_007() {
        assert_eq!(problem_007(10001), 104743);
    }
}
