struct Solution {}

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace().count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_segments() {
        assert_eq!(Solution::count_segments("Hello, my name is John".to_string()), 5);
        assert_eq!(Solution::count_segments("Hello, my name is John   ".to_string()), 5);
        assert_eq!(Solution::count_segments("Hello, my name is John \t  ".to_string()), 5);
    }
}
