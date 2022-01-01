#[derive(Debug)]
pub struct DisjointSet {
    n: usize,
    parents: Vec<usize>,
}

impl DisjointSet {
    pub fn from(n: usize) -> Self {
        let mut parents = Vec::with_capacity(n);
        for i in 0..n {
            parents.push(i);
        }
        DisjointSet { n, parents }
    }

    pub fn find(&mut self, p: usize) -> usize {
        if p != self.parents[p] {
            self.parents[p] = self.find(self.parents[p]);
            self.parents[p]
        } else {
            p
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pp = self.find(p);
        let pq = self.find(q);
        self.parents[pp] = pq;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disjoint_set_works() {
        let mut ds = DisjointSet::from(6);
        ds.union(0, 1);
        ds.union(1, 2);
        ds.union(3, 5);
        ds.union(3, 4);

        assert_eq!(ds.find(0), ds.find(2));
        assert_eq!(ds.find(4), ds.find(5));

        // println!("{:?}", ds);
    }
}
