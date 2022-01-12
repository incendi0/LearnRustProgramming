struct Solution;
impl Solution {

    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        let (mut a, mut b) = (i32::MAX, i32::MAX);
        for num in nums {
            if a >= num {
                a = num
            } else if b >= num {
                b = num;
            } else if b < num {
                return true;
            }
        }
        false
    }
}
