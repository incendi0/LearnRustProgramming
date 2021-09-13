use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;
        for i in 0..points.len() {
            for j in 0..points.len() {
                if i == j {
                    continue;
                }
                let dx = points[i][0] - points[j][0];
                let dy = points[i][1] - points[j][1];
                let distance = dx * dx + dy * dy;
                let c = count.entry(distance).or_insert(0);
                ret += *c;
                *c += 1;
            }
            count.clear();
        }
        ret * 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_number_of_boomerangs() {
        assert_eq!(
            Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]),
            2
        );
        assert_eq!(
            Solution::number_of_boomerangs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            2
        );
        assert_eq!(Solution::number_of_boomerangs(vec![vec![1, 1]]), 0);
    }
}
