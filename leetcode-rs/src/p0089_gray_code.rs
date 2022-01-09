struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut codes = vec![0, 1];
        for i in 1..n {
            let len = codes.len();
            for j in (0..len).rev() {
                codes.push(codes[j] | (1 << i));
            }
        }
        codes
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_peak_element() {
        assert_eq!(Solution::gray_code(1), [0, 1]);
        assert_eq!(Solution::gray_code(2), [0, 1, 3, 2]);
    }
}
