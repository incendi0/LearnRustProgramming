use std::collections::HashMap;

use super::{Color, DfsVertex};

// dfs used to search all paths, we just print out final graph
pub fn depth_first_search(graph: &Vec<Vec<usize>>) -> Vec<DfsVertex> {
    let n = graph.len();
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert(i, DfsVertex::new(i));
    }
    let mut time = 0;
    for i in 0..n {
        if map.get(&i).unwrap().color == Color::White {
            dfs(graph, i,  &mut map, &mut time);
        }
    }
    map.into_iter().map(|(_, v)| v).collect()
}

fn dfs(graph: &Vec<Vec<usize>>, u: usize, map: &mut HashMap<usize, DfsVertex>, time: &mut usize) {
    *time += 1;
    let vertex = map.get_mut(&u).unwrap();
    vertex.color = Color::Gray;
    vertex.first_visit_time = *time;
    for &v in graph[u].iter() {
        if map.get(&v).unwrap().color == Color::White {
            map.get_mut(&v).unwrap().predecessor = u;
            dfs(graph, v, map, time);
        }
    }
    *time += 1;
    let vertex = map.get_mut(&u).unwrap();
    vertex.second_visit_time = *time;
    vertex.color = Color::Black;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_maximum_works() {
        let graph = vec![vec![1], vec![0, 2], vec![1, 3], vec![2, 4, 5], vec![3, 5, 6, 7], vec![3, 4, 6], vec![4, 5, 7], vec![4, 6]];
        let vs = depth_first_search(&graph);
        println!("{:?}", vs);
    }
}
