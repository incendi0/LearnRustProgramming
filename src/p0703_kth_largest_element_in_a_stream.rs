use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    topk: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut r = KthLargest {
            topk: BinaryHeap::new(),
            k: k as usize,
        };
        nums.into_iter().for_each(|num| {
            r.add(num);
        });
        r
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.topk.len() < self.k {
            self.topk.push(Reverse(val));
        } else {
            if let Some(Reverse(mini)) = self.topk.peek() {
                if *mini < val {
                    self.topk.pop();
                    self.topk.push(Reverse(val));
                }
            }
        }
        self.topk.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::KthLargest;

    #[test]
    fn test_kth_largest() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }
}
