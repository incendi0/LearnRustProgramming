use crate::disjoint_set::DisjointSet;

#[derive(Debug)]
pub struct Edge {
    u: usize,
    v: usize,
    w: usize,
}

pub fn mst_kruskal(edges: Vec<Edge>, n: usize) -> Vec<Edge> {
    let mut ds = DisjointSet::from(n);
    let mut edges = edges;
    let mut mst = vec![];
    edges.sort_by_key(|edge| edge.w);
    for edge @ Edge { u, v, w: _ } in edges {
        if ds.find(u) != ds.find(v) {
            mst.push(edge);
            ds.union(u, v);
        }
    }
    mst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mst_kruskal_works() {
        let edges = vec![
            Edge { u: 0, v: 1, w: 4 },
            Edge { u: 0, v: 7, w: 8 },
            Edge { u: 1, v: 7, w: 11 },
            Edge { u: 1, v: 2, w: 8 },
            Edge { u: 2, v: 3, w: 7 },
            Edge { u: 2, v: 8, w: 2 },
            Edge { u: 2, v: 5, w: 4 },
            Edge { u: 3, v: 4, w: 9 },
            Edge { u: 3, v: 5, w: 14 },
            Edge { u: 4, v: 5, w: 10 },
            Edge { u: 5, v: 6, w: 2 },
            Edge { u: 6, v: 7, w: 1 },
            Edge { u: 6, v: 8, w: 6 },
            Edge { u: 7, v: 8, w: 7 },
        ];
        let mst = mst_kruskal(edges, 9);
        assert_eq!(mst.iter().map(|e| e.w).sum::<usize>(), 37);
    }
}
