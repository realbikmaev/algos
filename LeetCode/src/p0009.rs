// 9. Palindrome Number

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut l = x;
        let mut r = 0;
        while l > 0 {
            r = r * 10 + l % 10;
            l /= 10;
            println!("l: {l} r: {r}");
        }
        x == r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_is_palindrome() {
        assert_eq!(Solution::is_palindrome(12321), true);
    }
    #[test]
    fn check_is_palindrome_2() {
        assert_eq!(Solution::is_palindrome(12322), false);
    }
}
