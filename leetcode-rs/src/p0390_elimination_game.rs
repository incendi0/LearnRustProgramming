struct Solution;

impl Solution {
    /// 1 2 3 4 5 6 7
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            1
        } else {
            2 * Self::last_remaining_from_right(n / 2)
        }
    }

    fn last_remaining_from_right(n: i32) -> i32 {
        if n == 1 {
            1
        } else {
            if n % 2 == 1 {
                2 * Self::last_remaining(n / 2)
            } else {
                2 * Self::last_remaining(n / 2) - 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_remaining_works() {
        assert_eq!(Solution::last_remaining(9), 6);
        assert_eq!(Solution::last_remaining(1), 1);
    }
}
