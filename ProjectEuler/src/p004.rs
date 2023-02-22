use crate::utils::is_palindrome_int;

fn two_mul(n_digits: i64) -> impl Iterator<Item = i64> {
    let lo = 10i64.pow((n_digits - 1) as u32);
    let hi = lo * 10_i64;
    let mut i = 0;
    let mut j = 0;
    std::iter::from_fn(move || {
        let res: i64 = i * j;
        if j < hi {
            j += 1;
        }
        if j == hi {
            i += 1;
            j = i;
        }
        if i == hi {
            return None;
        }
        Some(res)
    })
}

pub fn problem_004(n_digits: i64) -> i64 {
    let res = two_mul(n_digits)
        .filter(|x| is_palindrome_int(*x))
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
