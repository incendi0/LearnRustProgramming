#![allow(dead_code)]

use std::collections::{BinaryHeap, HashSet};

mod first;
mod fourth;
mod second;
mod third;

struct Solution;

/**
给定两个以升序排列的整数数组 nums1 和 nums2 , 以及一个整数 k 。

定义一对值 (u,v)，其中第一个元素来自 nums1，第二个元素来自 nums2 。

请找到和最小的 k 个数对 (u1,v1),  (u2,v2)  ...  (uk,vk) 。



示例 1:

输入: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
输出: [1,2],[1,4],[1,6]
解释: 返回序列中的前 3 对数：
     [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]

示例 2:

输入: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
输出: [1,1],[1,1]
解释: 返回序列中的前 2 对数：
     [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]

示例 3:

输入: nums1 = [1,2], nums2 = [3], k = 3
输出: [1,3],[2,3]
解释: 也可能序列中所有的数对都被返回:[1,3],[2,3]



提示:

    1 <= nums1.length, nums2.length <= 104
    -109 <= nums1[i], nums2[i] <= 109
    nums1, nums2 均为升序排列
    1 <= k <= 1000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-k-pairs-with-smallest-sums
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (nums1.len(), nums2.len());
        let mut pq = BinaryHeap::new();
        let mut ret = vec![];
        let mut visited = HashSet::new();
        pq.push((-(nums1[0] + nums2[0]), 0, 0));
        visited.insert((0, 0));
        while !pq.is_empty() {
            let (_, i, j) = pq.pop().unwrap();
            ret.push(vec![nums1[i], nums2[j]]);
            if ret.len() == k as usize {
                break;
            }
            let ns = vec![(i + 1, j), (i, j + 1)];
            for (ni, nj) in ns {
                if ni < m && nj < n && !visited.contains(&(ni, nj)) {
                    pq.push((-(nums1[ni] + nums2[nj]), ni, nj));
                    visited.insert((ni, nj));
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn k_smallest_pairs_works() {
        assert_eq!(Solution::k_smallest_pairs(vec![1,7,11], vec![2, 4, 6], 3), [[1,2],[1,4],[1,6]]);
        assert_eq!(Solution::k_smallest_pairs(vec![1,1,2], vec![1, 2, 3], 2), [[1,1],[1,1]]);
        assert_eq!(Solution::k_smallest_pairs(vec![1,2], vec![3], 3), [[1,3],[2,3]]);
    }
}
