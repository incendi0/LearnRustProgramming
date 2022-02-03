struct Solution;

impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let (mut a, mut b) = (1, 1);
        while b <= k {
            let c = a + b;
            a = b;
            b = c;
        }
        let mut ret = 0;
        let mut k = k;
        while k > 0 {
            if k >= b {
                k -= b;
                ret += 1;
            }
            let c = b - a;
            b = a;
            a = c;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_min_fibonacci_numbers_works() {
        assert_eq!(Solution::find_min_fibonacci_numbers(7), 2);
        assert_eq!(Solution::find_min_fibonacci_numbers(10), 2);
        assert_eq!(Solution::find_min_fibonacci_numbers(19), 3);
    }
}