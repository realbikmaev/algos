use gen_iter::gen_iter;

fn is_palindrome(n: i64) -> bool {
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

fn two_mul_iter(n_digits: i64) -> impl Iterator<Item = i64> {
    gen_iter!(move {
        let lo = 10_i64.pow((n_digits - 1) as u32);
        let hi = lo * 10_i64;
        for i in lo..hi {
            for j in i..hi {
                yield i * j;
            }
        }
    })
}

pub fn problem_004(n_digits: i64) -> i64 {
    let res = two_mul_iter(n_digits)
        .filter(|x| is_palindrome(*x))
        .max()
        .unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_004() {
        assert_eq!(problem_004(2), 9009);
    }
    #[test]
    fn verify_004() {
        assert_eq!(problem_004(3), 906609);
    }
}
