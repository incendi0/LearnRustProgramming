use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct HuffmanNode {
    val: String,
    freq: i32,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.freq == other.freq
    }
}

impl Eq for HuffmanNode {}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl HuffmanNode {
    fn from_sons(lhs: HuffmanNode, rhs: HuffmanNode) -> HuffmanNode {
        HuffmanNode {
            val: format!("{}{}", lhs.val, rhs.val),
            freq: lhs.freq + rhs.freq,
            left: Some(Box::new(lhs)),
            right: Some(Box::new(rhs)),
        }
    }

    fn from(vch: char, freq: i32) -> HuffmanNode {
        HuffmanNode {
            val: String::from(vch),
            freq,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct HuffmanTree {
    root: Option<Box<HuffmanNode>>,
}

pub fn construct_huffmantree(xs: &Vec<(char, i32)>) -> HuffmanTree {
    if xs.is_empty() {
        return HuffmanTree { root: None };
    }
    let nodes: Vec<HuffmanNode> = xs
        .iter()
        .map(|&(vch, f)| HuffmanNode::from(vch, f))
        .collect();
    let mut pq = BinaryHeap::from(nodes);
    construct_huffmantree_aux(&mut pq);
    HuffmanTree {
        root: Some(Box::new(pq.pop().unwrap())),
    }
}

fn construct_huffmantree_aux(pq: &mut BinaryHeap<HuffmanNode>) {
    let mut n = pq.len();
    while n > 1 {
        for _ in 0..n - 1 {
            let lhs = pq.pop().unwrap();
            let rhs = pq.pop().unwrap();
            let new_node = HuffmanNode::from_sons(lhs, rhs);
            pq.push(new_node);
        }
        n = pq.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_huffmantree_works() {
        let xs = vec![
            ('f', 5),
            ('e', 9),
            ('c', 12),
            ('b', 13),
            ('d', 16),
            ('a', 45),
        ];
        let ht = construct_huffmantree(&xs);
        assert!(ht.root.is_some());
        assert_eq!(ht.root.unwrap().freq, 100);
    }
}
