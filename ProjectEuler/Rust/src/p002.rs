fn fib_iter(below: i64) -> impl Iterator<Item = i64> {
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

pub fn problem_002(below: i64) -> i64 {
    let res = fib_iter(below).filter(|x| x % 2 == 0).sum();
    print!("{res}");
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_002() {
        assert_eq!(problem_002(4_000_000), 4613732);
    }
    #[test]
    fn verify_002() {
        assert_eq!(problem_002(50), 44);
    }
}
