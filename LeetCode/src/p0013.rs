// 13. Roman to Integer

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int_fast(s: String) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut arr = [0_i32; 100];
        let mut len = 0;
        s.chars().into_iter().for_each(|c| {
            arr[len] = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };
            len += 1;
        });

        while i < len {
            match arr[i] {
                1 => {
                    if i + 1 < len {
                        match arr[i + 1] {
                            5 => {
                                ans += 4;
                                i += 2;
                            }
                            10 => {
                                ans += 9;
                                i += 2;
                            }
                            _ => {
                                ans += 1;
                                i += 1;
                            }
                        }
                    } else {
                        ans += 1;
                        i += 1;
                    }
                }
                5 => {
                    ans += 5;
                    i += 1;
                }
                10 => {
                    if i + 1 < len {
                        match arr[i + 1] {
                            50 => {
                                ans += 40;
                                i += 2;
                            }
                            100 => {
                                ans += 90;
                                i += 2;
                            }
                            _ => {
                                ans += 10;
                                i += 1;
                            }
                        }
                    } else {
                        ans += 10;
                        i += 1;
                    }
                }
                50 => {
                    ans += 50;
                    i += 1;
                }
                100 => {
                    if i + 1 < len {
                        match arr[i + 1] {
                            500 => {
                                ans += 400;
                                i += 2;
                            }
                            1000 => {
                                ans += 900;
                                i += 2;
                            }
                            _ => {
                                ans += 100;
                                i += 1;
                            }
                        }
                    } else {
                        ans += 100;
                        i += 1;
                    }
                }
                500 => {
                    ans += 500;
                    i += 1;
                }
                1000 => {
                    ans += 1000;
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
        ans
    }

    pub fn roman_to_int(s: String) -> i32 {
        let ri = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut sum: i32 = 0;

        let arr: Vec<i32> = s.chars().into_iter().map(|c| ri[&c]).collect();

        let penultimate = arr.len() - 1;
        for (i, a) in arr.iter().enumerate() {
            match i {
                _ if i == penultimate => sum += a,
                _ if a < &arr[i + 1] => sum -= a,
                _ => sum += a,
            }
        }
        sum
    }

    pub fn roman_to_int_2(s: String) -> i32 {
        let ri = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut sum: i32 = 0;

        let arr: Vec<i32> = s.chars().into_iter().map(|c| ri[&c]).chain(0..1).collect();

        for i in 0..arr.len() - 1 {
            match i {
                _ if arr[i] < arr[i + 1] => sum -= arr[i],
                _ => sum += arr[i],
            }
        }
        sum
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num1) in nums.iter().enumerate() {
            let num2 = target - num1;
            if map.contains_key(&num2) {
                return vec![map[&num2], i as i32];
            }
            map.insert(num1, i as i32);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_ri() {
        assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
    }

    #[test]
    fn check_ri_2() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
    }

    #[test]
    fn check_ri_3() {
        assert_eq!(Solution::roman_to_int_fast("MCMXCIV".into()), 1994);
    }

    #[test]
    fn check_ri_4() {
        assert_eq!(Solution::roman_to_int_2("III".into()), 3);
    }
    #[test]
    fn check_ri_5() {
        assert_eq!(Solution::roman_to_int_fast("XXX".into()), 30);
    }

    #[test]
    fn check_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn check_two_sum_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
