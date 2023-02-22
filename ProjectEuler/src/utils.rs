pub fn fib_iter(below: i64) -> impl Iterator<Item = i64> {
    let mut a = 0;
    let mut b = 1;
    std::iter::from_fn(move || {
        let c = a + b;
        a = b;
        b = c;
        if c > below {
            None
        } else {
            Some(c)
        }
    })
}

pub fn prime_factors_of(n: i64) -> Vec<i64> {
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

pub fn is_palindrome_int(n: i64) -> bool {
    let s = n
        .to_string()
        .chars()
        .into_iter()
        .rev()
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    s == n
}

pub fn is_divisible_by_all(n: i64, divisors: &[i64]) -> bool {
    for d in divisors {
        if n % d != 0 {
            return false;
        }
    }
    true
}
