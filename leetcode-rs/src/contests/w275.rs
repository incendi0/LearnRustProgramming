use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        let mut f = vec![0; n];
        for row in &matrix {
            f.fill(0);
            for &v in row {
                let v = v as usize;
                f[v - 1] += 1;
                if f[v - 1] > 1 {
                    return false;
                }
            }
        }

        for j in 0..n {
            f.fill(0);
            for i in 0..n {
                let v = matrix[i][j] as usize;
                f[v - 1] += 1;
                if f[v - 1] > 1 {
                    return false;
                }
            }
        }
        true
    }

    // 一开始理解成了聚集到左右两侧，其实中间也可以，打了个补丁
    // 滑动窗口多往右滑total-1步就行，注意下标判断
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        use std::cmp::min;
        let n = nums.len();
        let mut prefix_sum = vec![0; n];
        prefix_sum[0] = nums[0];
        let mut suffix_sum = vec![0; n];
        suffix_sum[n - 1] = nums[n - 1];
        let mut total = nums[0];
        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i];
            let idx = n - i - 1;
            suffix_sum[idx] = suffix_sum[idx + 1] + nums[idx];
            total += nums[i];
        }
        if total == 0 {
            return 0;
        }
        let mut ret = min(
            total - prefix_sum[total as usize - 1],
            total - suffix_sum[n - total as usize],
        );
        for l in 1..total {
            let r = total - l;
            ret = min(
                ret,
                total - prefix_sum[l as usize - 1] - suffix_sum[n - r as usize],
            );
        }

        for m in total as usize..n {
            ret = min(ret, total - prefix_sum[m] + prefix_sum[m - total as usize]);
        }
        ret
    }

    // 大意了，95行的数加成了频率 :(
    pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
        let mut count = HashMap::new();
        for s in start_words {
            let mut r = 0;
            for ch in s.bytes() {
                let idx = (ch - b'a') as i32;
                r += 2i32.pow((26 - idx - 1) as u32);
            }
            *count.entry(r).or_insert(0) += 1;
        }
        let mut ret = 0;
        for s in target_words {
            let n = s.len();
            if n == 1 {
                continue;
            }
            for i in 0..n {
                let mut r = 0;
                for (j, ch) in s.bytes().enumerate() {
                    if j == i {
                        continue;
                    }
                    let idx = (ch - b'a') as i32;
                    r += 2i32.pow((26 - idx - 1) as u32);
                }
                if count.contains_key(&r) {
                    ret += 1;
                    break;
                }
            }
        }
        ret
    }

    // 贪心算法，面对贪心总是无力
    // grow time长的先种上
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        use std::cmp::max;
        let n = plant_time.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| grow_time[b].cmp(&grow_time[a]));
        let mut time = 0;
        let mut ret = 0;
        for i in order {
            time += plant_time[i];
            ret = max(ret, time + grow_time[i]);
        }
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;


    #[test] 
    fn earliest_full_bloom_works() {
        assert_eq!(Solution::earliest_full_bloom(vec![1,4,3], vec![2,3,1]), 9);
        assert_eq!(Solution::earliest_full_bloom(vec![1,2,3,2], vec![2,1,2,1]), 9);
        assert_eq!(Solution::earliest_full_bloom(vec![1], vec![1]), 2);
    }

    #[test]
    fn min_swaps_works() {
        assert_eq!(Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
        assert_eq!(Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
        assert_eq!(Solution::min_swaps(vec![1, 1, 0, 0, 1]), 0);
        assert_eq!(Solution::min_swaps(vec![0, 1, 0, 0, 1, 0, 0, 0, 1]), 1);
    }

    #[test]
    fn word_count_works() {
        assert_eq!(
            Solution::word_count(
                vec!["ant".into(), "act".into(), "tack".into()],
                vec!["tack".into(), "act".into(), "acti".into()]
            ),
            2
        );
        assert_eq!(
            Solution::word_count(
                vec!["ab".into(), "a".into()],
                vec!["abc".into(), "abcd".into()]
            ),
            1
        );
        assert_eq!(
            Solution::word_count(
                vec![
                    "g".into(),
                    "vf".into(),
                    "ylpuk".into(),
                    "nyf".into(),
                    "gdj".into(),
                    "j".into(),
                    "fyqzg".into(),
                    "sizec".into()
                ],
                vec![
                    "r".into(),
                    "am".into(),
                    "jg".into(),
                    "umhjo".into(),
                    "fov".into(),
                    "lujy".into(),
                    "b".into(),
                    "uz".into(),
                    "y".into()
                ]
            ),
            2
        );
    }
}
