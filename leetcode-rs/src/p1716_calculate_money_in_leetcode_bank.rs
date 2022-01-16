struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let first_week_sum = 28;
        let m = n / 7;
        let h = n % 7;
        (first_week_sum + (m - 1) * 7 + first_week_sum) * m / 2 + (1 + m + 1 + m + h - 1) * h / 2
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn total_money_works() {
        assert_eq!(Solution::total_money(4), 10);
        assert_eq!(Solution::total_money(10), 37);
        assert_eq!(Solution::total_money(20), 96);
    }
}
