use std::cmp::min;

pub fn mst_prim(graph: &Vec<Vec<usize>>) -> usize {
    let n = graph.len();
    let mut distance = vec![usize::MAX; n];
    let mut in_mst = vec![false; n];
    let mut ret = 0;
    distance[0] = 0;
    for _ in 0..n {
        let (mut next_vertex, mut next_edge_length) = (n, usize::MAX);
        for j in 0..n {
            if !in_mst[j] && distance[j] < next_edge_length {
                next_vertex = j;
                next_edge_length = distance[j];
            }
        }
        in_mst[next_vertex] = true;
        ret += next_edge_length;

        for j in 0..n {
            if !in_mst[j] {
                distance[j] = min(distance[j], graph[next_vertex][j]);
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mst_prim_works() {
        let graph = vec![
            vec![
                100000000, 4, 100000000, 100000000, 100000000, 100000000, 100000000, 8, 100000000,
            ],
            vec![
                4, 100000000, 8, 100000000, 100000000, 100000000, 100000000, 11, 100000000,
            ],
            vec![
                100000000, 8, 100000000, 7, 100000000, 4, 100000000, 100000000, 2,
            ],
            vec![
                100000000, 100000000, 7, 100000000, 9, 14, 100000000, 100000000, 100000000,
            ],
            vec![
                100000000, 100000000, 100000000, 9, 100000000, 10, 100000000, 100000000, 100000000,
            ],
            vec![
                100000000, 100000000, 4, 14, 10, 100000000, 2, 100000000, 100000000,
            ],
            vec![
                100000000, 100000000, 100000000, 100000000, 100000000, 2, 100000000, 1, 6,
            ],
            vec![
                8, 11, 100000000, 100000000, 100000000, 100000000, 1, 100000000, 7,
            ],
            vec![
                100000000, 100000000, 2, 100000000, 100000000, 100000000, 6, 7, 100000000,
            ],
        ];
        assert_eq!(mst_prim(&graph), 37);
    }
}
