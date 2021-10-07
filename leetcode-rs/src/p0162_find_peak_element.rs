struct Solution {}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + r >> 1;
            if nums[m] > nums[m + 1] {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_peak_element() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::find_peak_element(vec![5, 4, 3, 2, 1]), 0);

        let peak_idx = Solution::find_peak_element(vec![1, 4, 2, 7, 5, 4, 6]);
        let peak_indices = vec![1, 3, 6];
        assert!(peak_indices.iter().position(|&x| x == peak_idx).is_some());
    }
}
