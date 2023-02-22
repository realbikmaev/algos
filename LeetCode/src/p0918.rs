// 918. Maximum Sum Circular Subarray

pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let mut max_so_far = i32::MIN;
    let mut max_ending_here = 0;
    let len = nums.len();
    for i in 0..len {
        max_ending_here = max_ending_here + nums[i];
        if max_so_far < max_ending_here {
            max_so_far = max_ending_here;
        }
        if max_ending_here < 0 {
            max_ending_here = 0;
        }
    }
    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_ms_1() {
        assert_eq!(max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
    }
    #[test]
    fn check_ms_2() {
        assert_eq!(max_subarray_sum_circular(vec![5, -3, 5]), 10);
    }
    #[test]
    fn check_ms_3() {
        assert_eq!(max_subarray_sum_circular(vec![-3, -2, -3]), -2);
    }
}
