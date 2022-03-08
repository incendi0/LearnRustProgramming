use core::num;

struct Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|w| w.starts_with(&pref)).count() as i32
    }

    pub fn min_steps(s: String, t: String) -> i32 {
        let mut count: Vec<i32> = vec![0; 26];
        for &ch in s.as_bytes() {
            count[(ch - b'a') as usize] += 1;
        }
        for &ch in t.as_bytes() {
            count[(ch - b'a') as usize] -= 1;
        }
        count.into_iter().map(|d| d.abs()).sum()
    }

    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut l = 1i64;
        let mut r = *time.iter().max().unwrap() as i64 * total_trips as i64;
        while l < r {
            let m = (l + r) / 2;
            let mut trips = 0;
            for t in &time {
                trips += m / *t as i64;
            }
            if trips >= total_trips as i64 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }

    // 1. 计算出跑i圈使用一个轮胎所需要最短的时间，使用min_time[i]表示
    // 2. 对于第i圈需要的最短时间 f(i) = min(f(i), f(i - j) + change_time + min_time(j)) for j in 0..=i
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        use std::cmp::min;
        let mut min_time = vec![i32::MAX as i64; 21];
        for (i, tire) in tires.iter().enumerate() {
            let mut elapsed: i64 = 0;
            for j in 1..min_time.len() {
                elapsed = elapsed * (tire[1] as i64) + tire[0] as i64;
                if elapsed > i32::MAX as i64 {
                    break;
                }
                min_time[j] = min(min_time[j], elapsed);
            }
        }    
        println!("min_tim: {:?}", min_time);
        let mut f = vec![i32::MAX as i64; (num_laps + 1) as usize];
        f[0] = -change_time as i64;
        for i in 1..=num_laps as usize {
            for j in 1..=min(i, 20) {
                f[i] = min(f[i], f[i - j] + min_time[j] + change_time as i64);
            }
        }
        f[num_laps as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn prefix_count_works() {
        assert_eq!(Solution::prefix_count(vec!["pay","attention","practice","attend"].into_iter().map(|s| s.to_string()).collect(), "at".to_string()), 2);
        assert_eq!(Solution::prefix_count(vec!["leetcode","win","loops","success"].into_iter().map(|s| s.to_string()).collect(), "code".to_string()), 0);
    }

    #[test]
    fn min_steps_works() {
        assert_eq!(Solution::min_steps("leetcode".to_string(), "coats".to_string()), 7);
        assert_eq!(Solution::min_steps("night".to_string(), "thing".to_string()), 0);
    }

    #[test]
    fn minimum_time_works() {
        assert_eq!(Solution::minimum_time(vec![1, 2, 3], 5), 3);
        assert_eq!(Solution::minimum_time(vec![2], 1), 2);
        assert_eq!(Solution::minimum_time(vec![9,7,10,9,10,9,10], 1), 7);
    }

    #[test]
    fn minimum_finish_time_works() {
        assert_eq!(Solution::minimum_finish_time(vec![vec![2,3],vec![3,4]], 5, 4), 21);
        assert_eq!(Solution::minimum_finish_time(vec![vec![1,10],vec![2,2],vec![3,4]], 6, 5), 25);
    }
}