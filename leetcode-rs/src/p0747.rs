struct Solution;
impl Solution {
    // 找最大次大值
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut big_idx = nums.len();
        let (mut big, mut second) = (i32::MIN, i32::MIN);
        for (i, &num) in nums.iter().enumerate() {
            if num >= big {
                second = big;
                big = num;
                big_idx = i;
            } else if num > second {
                second = num;
            }
        }
        if big >= second * 2 {
            big_idx as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn dominant_index_works() {
        assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
        assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
        assert_eq!(Solution::dominant_index(vec![1]), 0);
    }
}
