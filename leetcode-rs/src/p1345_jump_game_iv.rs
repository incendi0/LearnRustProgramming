use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut graph: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut not_visited: HashSet<i32> = HashSet::new();
        for (idx, &val) in arr.iter().enumerate() {
            let adj = graph.entry(val).or_insert(Vec::new());
            adj.push(idx);
            not_visited.insert(val);
        }
        let mut distance = vec![n + 1; n];
        distance[0] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            if u >= 1 && distance[u - 1] > distance[u] + 1 {
                distance[u - 1] = distance[u] + 1;
                queue.push_back(u - 1);
            }
            if u + 1 < n && distance[u + 1] > distance[u] + 1 {
                distance[u + 1] = distance[u] + 1;
                queue.push_back(u + 1);
            }
            if not_visited.contains(&arr[u]) {
                for &v in &graph[&arr[u]] {
                    if distance[v] > distance[u] + 1 {
                        distance[v] = distance[u] + 1;
                        queue.push_back(v);
                    }
                }
                not_visited.remove(&arr[u]);
            }
        }
        distance[n - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn min_jumps_works() {
        assert_eq!(
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
            3
        );
        assert_eq!(Solution::min_jumps(vec![7]), 0);
        assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
        assert_eq!(Solution::min_jumps(vec![6, 1, 9]), 2);
        assert_eq!(
            Solution::min_jumps(vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13]),
            3
        );
    }
}
