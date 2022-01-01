use std::collections::VecDeque;

pub fn toposort(xs: &Vec<Vec<usize>>, n: usize) -> Vec<usize> {
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut in_degrees = vec![0; n];
    for pair in xs.iter() {
        // assume u -> v
        let (u, v) = (pair[0], pair[1]);
        in_degrees[v] += 1;
        graph[u].push(v);
    }

    let mut ret = vec![];
    let mut queue = VecDeque::new();
    for i in 0..n {
        if in_degrees[i] == 0 {
            ret.push(i);
            queue.push_back(i);
        }
    }
    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();
        for &v in graph[u].iter() {
            in_degrees[v] -= 1;
            if in_degrees[v] == 0 {
                ret.push(v);
                queue.push_back(v);
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_maximum_works() {
        let xs = vec![
            vec![0, 1],
            vec![0, 4],
            vec![1, 2],
            vec![1, 4],
            vec![2, 7],
            vec![3, 4],
            vec![5, 2],
            vec![5, 6],
            vec![6, 7],
        ];
        let vs = toposort(&xs, 8);
        for pair in xs {
            let (u, v) = (pair[0], pair[1]);
            let u_idx = vs.iter().position(|&e| e == u).unwrap();
            let v_idx = vs.iter().position(|&e| e == v).unwrap();
            assert!(u_idx < v_idx);
        }
        // println!("{:?}", vs);
    }
}
