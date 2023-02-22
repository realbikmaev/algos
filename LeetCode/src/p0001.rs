// 1. Two Sum

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num1) in nums.iter().enumerate() {
            let num2 = target - num1;
            if map.contains_key(&num2) {
                return vec![map[&num2], i as i32];
            }
            map.insert(num1, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
