use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let ss = s.as_bytes().to_vec();
        let mut ret = vec![];
        for alpha in ss[0]..=ss[3] {
            for idx in ss[1]..=ss[4] {
                let mut cell = String::new();
                cell.push(alpha as char);
                cell.push(idx as char);
                ret.push(cell);
            }
        }
        ret
    }

    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut rhs = k + n as i32;
        let mut ret = rhs as i64 * (rhs as i64 + 1) / 2;
        let appeared: HashSet<i32> = nums.into_iter().filter(|&num| num <= rhs).collect();
        let mut count = 0;
        loop {
            if count == n - appeared.len() {
                break;
            }
            if !appeared.contains(&rhs) {
                ret -= rhs as i64;
                count += 1;
            }
            rhs -= 1;
        }
        ret - appeared.into_iter().map(|d| d as i64).sum::<i64>()
    }

    // 树结构，用Java写了
    // public TreeNode createBinaryTree(int[][] descriptions) {
    //     Set<Integer> possibleRoot = new HashSet<>();
    //     Set<Integer> impossibleRoot = new HashSet<>();
    //     Map<Integer, TreeNode> tree = new HashMap<>();
    //     for (int[] desc : descriptions) {
    //         int p = desc[0], s = desc[1];
    //         TreeNode parent = tree.getOrDefault(p, new TreeNode(p));
    //         TreeNode son = tree.getOrDefault(s, new TreeNode(s));
    //         if (desc[2] == 1) {
    //             parent.left = son;
    //         } else {
    //             parent.right = son;
    //         }
    //         tree.put(p, parent);
    //         tree.put(s, son);
    //         impossibleRoot.add(s);
    //         possibleRoot.remove(s);
    //         if (!impossibleRoot.contains(p)) {
    //             possibleRoot.add(p);
    //         }
    //     }
    //     return tree.get(possibleRoot.iterator().next());
    // }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn cells_in_range_works() {
        assert_eq!(
            Solution::cells_in_range("K1:L2".into()),
            ["K1", "K2", "L1", "L2"]
        );
        assert_eq!(
            Solution::cells_in_range("A1:F1".into()),
            ["A1", "B1", "C1", "D1", "E1", "F1"]
        );
    }

    #[test]
    fn minimal_k_sum_works() {
        assert_eq!(Solution::minimal_k_sum(vec![1, 4, 25, 10, 25], 2), 5);
        assert_eq!(Solution::minimal_k_sum(vec![5, 6], 6), 25);
        assert_eq!(
            Solution::minimal_k_sum(
                vec![53, 41, 90, 33, 84, 26, 50, 32, 63, 47, 66, 43, 29, 88, 71, 28, 83],
                76
            ),
            3444
        );
        assert_eq!(
            Solution::minimal_k_sum(
                vec![
                    96, 44, 99, 25, 61, 84, 88, 18, 19, 33, 60, 86, 52, 19, 32, 47, 35, 50, 94, 17,
                    29, 98, 22, 21, 72, 100, 40, 84
                ],
                35
            ),
            794
        );
    }
}
