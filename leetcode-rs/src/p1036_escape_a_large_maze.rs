struct Solution;
impl Solution {
    // bfs，增加访问点数控制
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        Self::is_escape_possible_aux(&blocked, source.clone(), target.clone())
            && Self::is_escape_possible_aux(&blocked, target, source)
    }

    fn is_escape_possible_aux(blocked: &Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        use std::collections::HashSet;
        use std::collections::VecDeque;
        const N: i32 = 1e6 as i32;
        const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let blocked_set: HashSet<(i32, i32)> = blocked
            .into_iter()
            .map(|point| (point[0], point[1]))
            .collect();
        let mut q = VecDeque::new();
        q.push_back((source[0], source[1]));
        let mut visited = HashSet::new();
        while !q.is_empty() {
            if visited.len() > blocked_set.len() * blocked_set.len() / 2 {
                return true;
            }
            let (x, y) = q.pop_front().unwrap();
            if x == target[0] && y == target[1] {
                return true;
            }
            for (dx, dy) in &DIRS {
                let np = (dx + x, dy + y);
                if np.0 >= 0
                    && np.0 < N
                    && np.1 >= 0
                    && np.1 < N
                    && !blocked_set.contains(&np)
                    && !visited.contains(&np)
                {
                    q.push_back(np);
                    visited.insert(np);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn is_escape_possible_works() {
        let blocks = vec![vec![0, 1], vec![1, 0]];
        let source = vec![0, 0];
        let target = vec![0, 2];
        assert!(!Solution::is_escape_possible(blocks, source, target));
    }

    #[test]
    fn is_escape_possible_works_1() {
        let blocks = vec![vec![10, 9], vec![9, 10], vec![10, 11], vec![11, 10]];
        let source = vec![0, 0];
        let target = vec![10, 10];
        assert!(!Solution::is_escape_possible(blocks, source, target));
    }
}
