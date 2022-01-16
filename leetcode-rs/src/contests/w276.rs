use std::{
    cmp::{max, min},
    collections::HashMap,
};

struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let xs: Vec<char> = s.chars().collect();
        let mut ret: Vec<String> = xs
            .chunks(k as usize)
            .into_iter()
            .map(|f| f.iter().collect::<String>())
            .collect();
        if xs.len() % k as usize != 0 {
            let last = ret.last_mut().unwrap();
            (0..(k - xs.len() as i32 % k)).for_each(|_| last.push(fill));
        }
        ret
    }

    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut ret = 0;
        let (mut target, mut max_doubles) = (target, max_doubles);
        loop {
            if target == 1 || max_doubles <= 0 {
                break;
            } else if target % 2 == 1 {
                target -= 1;
            } else {
                max_doubles -= 1;
                target /= 2;
            }
            ret += 1;
        }
        ret += target - 1;
        ret
    }

    // memo[i]表示从i开始能获得的最大分数
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut memo = HashMap::new();
        Self::memoization(&questions, 0, &mut memo)
    }

    fn memoization(questions: &Vec<Vec<i32>>, idx: usize, memo: &mut HashMap<usize, i64>) -> i64 {
        if idx >= questions.len() {
            return 0;
        }
        if memo.contains_key(&idx) {
            return memo[&idx];
        }

        let mut ret = questions[idx][0] as i64
            + Self::memoization(questions, idx + questions[idx][1] as usize + 1, memo);
        ret = max(ret, Self::memoization(questions, idx + 1, memo));
        memo.insert(idx, ret);
        ret
    }

    // 二分
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let (mut l, mut r) = (0i64, batteries.iter().fold(0i64, |acc, e| acc + *e as i64));
        while l < r {
            let m = (l + r + 1) / 2;
            let sum = batteries
                .iter()
                .fold(0i64, |acc, e| acc + min(*e as i64, m));
            // 如果该条件成立，总能找到一种方案满足同时使用m分钟
            if sum / n as i64 >= m {
                l = m;
            } else {
                r = m - 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn divide_string_works() {
        assert_eq!(
            Solution::divide_string("abcdefghi".into(), 3, 'x'),
            vec!["abc", "def", "ghi"]
        );
        assert_eq!(
            Solution::divide_string("abcdefghij".into(), 3, 'x'),
            vec!["abc", "def", "ghi", "jxx"]
        );
    }

    #[test]
    fn min_moves_works() {
        assert_eq!(Solution::min_moves(5, 0), 4);
        assert_eq!(Solution::min_moves(19, 2), 7);
        assert_eq!(Solution::min_moves(10, 4), 4);
    }

    #[test]
    fn most_points_works() {
        assert_eq!(
            Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
            5
        );
        assert_eq!(
            Solution::most_points(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5]
            ]),
            7
        );
    }

    #[test]
    fn max_run_time_works() {
        assert_eq!(Solution::max_run_time(2, vec![3, 3, 3]), 4);
        assert_eq!(Solution::max_run_time(2, vec![1, 1, 1, 1]), 2);
    }
}
