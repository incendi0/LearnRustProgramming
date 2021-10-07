use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    /// 贪心（O(nlogn)）
    /// 1. 按照所需资本排序利润
    /// 2. 维护一个大根堆，不断将排序后所需资本小于当前w的利润放入堆中
    /// 3. k次从堆顶获取最大利润，并累加到w中
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut xs: Vec<(i32, i32)> = capital.into_iter().zip(profits).collect();
        xs.sort_by_key(|k| k.0);
        let mut pq = BinaryHeap::new();
        let mut idx = 0;
        let mut ret = w;
        for _ in 0..k {
            while idx < n && ret >= xs[idx].0 {
                pq.push(xs[idx].1);
                idx += 1;
            }
            if let Some(large) = pq.pop() {
                ret += large;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    fn it_works() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
        assert_eq!(
            Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
            6
        );
    }
}
