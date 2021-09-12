struct Solution {}

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let n = chalk.len();
        let mut sum = Vec::with_capacity(n + 1);
        sum.push(0u64);
        for i in 1..n + 1 {
            sum.push(sum.last().unwrap() + chalk[i - 1] as u64);
        }
        let (mut l, mut r) = (0, n);
        let target = k as u64 % sum.last().unwrap();
        while l < r {
            let m = (l + r + 1) >> 1;
            if sum[m] > target {
                r = m - 1;
            } else {
                l = m;
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_kth_largest() {
        assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0)
    }
}
