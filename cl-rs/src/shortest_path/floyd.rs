use std::cmp::min;

pub fn floyd(dist: &mut Vec<Vec<i32>>) {
    let n = dist.len();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                dist[j][k] = min(dist[j][k], dist[j][i] + dist[i][k]);
            }
        }
    }
}
