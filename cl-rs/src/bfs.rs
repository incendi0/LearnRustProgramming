use std::collections::{HashMap, VecDeque};

use super::{Color, BfsVertex};

// adjacent list
pub fn breadth_first_search(graph: &Vec<Vec<usize>>, start: usize, target: usize) -> bool {
    let n = graph.len();
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert(i, BfsVertex::new(i));
    }
    let mut start_vertex = map.get_mut(&start).unwrap();
    start_vertex.color = Color::Gray;
    start_vertex.distance = 0;

    let mut queue = VecDeque::new();
    queue.push_back(start);
    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();
        if u == target {
            for i in 0..n {
                println!("{:?}", map.get(&i));
            }
            return true;
        }
        let d = map.get(&u).unwrap().distance;
        for &v in graph[u].iter() {
            if map.get(&v).unwrap().color == Color::White {
                let vertex = map.get_mut(&v).unwrap();
                vertex.color = Color::Gray;
                vertex.distance = d + 1;
                vertex.predecessor = u;
                queue.push_back(v);
            }
        }
        map.get_mut(&u).unwrap().color = Color::Black;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_maximum_works() {
        let graph = vec![vec![1], vec![0, 2], vec![1, 3], vec![2, 4, 5], vec![3, 5, 6, 7], vec![3, 4, 6], vec![4, 5, 7], vec![4, 6]];
        let start = 2;
        let target = 7;
        assert!(breadth_first_search(&graph, start, target));
    }
}
