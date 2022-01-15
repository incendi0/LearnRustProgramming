#![allow(dead_code)]

use std::collections::{BinaryHeap, HashSet};

struct Solution;

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
