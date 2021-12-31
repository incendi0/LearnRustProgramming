// based on heap_sort
// 大顶堆
// backed by Vec
use super::sort::heap_sort::*;
pub fn heap_maximum<T: Ord>(xs: &mut Vec<T>) -> Option<&T> {
    xs.get(0)
}

pub fn heap_extract_max<T: Ord>(xs: &mut Vec<T>) -> Option<T> {
    if xs.len() == 0 {
        None
    } else {
        let n = xs.len();
        xs.swap(0, n - 1);
        max_heapify(xs, 0, n - 1);
        xs.pop()
    }
}

pub fn max_heap_insert<T: Ord>(xs: &mut Vec<T>, key: T) {
    xs.push(key);
    let n = xs.len();
    if n <= 1 {
        return;
    }
    let mut i = n - 1;
    while i > 0 && xs[parent(i)] < xs[i] {
        xs.swap(parent(i), i);
        i = parent(i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::priority_queue::max_heap_insert;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn heap_maximum_works() {
        let mut xs = vec![1, 2, 3, 4, 5, 6, 7, 8];
        xs.shuffle(&mut thread_rng());
        let mut heap = vec![];
        assert_eq!(heap_maximum(&mut heap), None);
        for x in xs {
            max_heap_insert(&mut heap, x)
        }
        assert_eq!(heap_maximum(&mut heap), Some(&8));
    }

    #[test]
    fn heap_extract_max_works() {
        for _ in 0..1000 {
            let mut xs = vec![1, 2, 3, 4, 5, 6, 7, 8];
            xs.shuffle(&mut thread_rng());
            let mut heap = vec![];
            assert_eq!(heap_extract_max(&mut heap), None);
            let cp = xs.clone();
            for x in xs {
                max_heap_insert(&mut heap, x);
            }
            let heap_cp = heap.clone();
            assert_eq!(
                heap_extract_max(&mut heap),
                Some(8),
                "input: {:?}, heap: {:?}",
                cp,
                heap_cp
            );
            assert_eq!(
                heap_extract_max(&mut heap),
                Some(7),
                "input: {:?}, heap: {:?}",
                cp,
                heap_cp
            );
            assert_eq!(
                heap_extract_max(&mut heap),
                Some(6),
                "input: {:?}, heap: {:?}",
                cp,
                heap_cp
            );
            assert_eq!(
                heap_extract_max(&mut heap),
                Some(5),
                "input: {:?}, heap: {:?}",
                cp,
                heap_cp
            );
            assert_eq!(
                heap_extract_max(&mut heap),
                Some(4),
                "input: {:?}, heap: {:?}",
                cp,
                heap_cp
            );
            assert_eq!(
                heap_extract_max(&mut heap),
                Some(3),
                "input: {:?}, heap: {:?}",
                cp,
                heap_cp
            );
            assert_eq!(
                heap_extract_max(&mut heap),
                Some(2),
                "input: {:?}, heap: {:?}",
                cp,
                heap_cp
            );
            assert_eq!(
                heap_extract_max(&mut heap),
                Some(1),
                "input: {:?}, heap: {:?}",
                cp,
                heap_cp
            );
            assert_eq!(heap_extract_max(&mut heap), None);
        }
    }

    #[test]
    fn heap_extract_max_works_1() {
        let xs = vec![5, 7, 3, 1, 8, 6, 2, 4];
        let mut heap = vec![];
        let cp = xs.clone();
        for x in xs {
            max_heap_insert(&mut heap, x);
        }
        let heap_cp = heap.clone();
        assert_eq!(
            heap_extract_max(&mut heap),
            Some(8),
            "input: {:?}, heap: {:?}",
            cp,
            heap_cp
        );
        assert_eq!(
            heap_extract_max(&mut heap),
            Some(7),
            "input: {:?}, heap: {:?}",
            cp,
            heap_cp
        );
        assert_eq!(
            heap_extract_max(&mut heap),
            Some(6),
            "input: {:?}, heap: {:?}",
            cp,
            heap_cp
        );
        assert_eq!(
            heap_extract_max(&mut heap),
            Some(5),
            "input: {:?}, heap: {:?}",
            cp,
            heap_cp
        );
        assert_eq!(
            heap_extract_max(&mut heap),
            Some(4),
            "input: {:?}, heap: {:?}",
            cp,
            heap_cp
        );
        assert_eq!(
            heap_extract_max(&mut heap),
            Some(3),
            "input: {:?}, heap: {:?}",
            cp,
            heap_cp
        );
        assert_eq!(
            heap_extract_max(&mut heap),
            Some(2),
            "input: {:?}, heap: {:?}",
            cp,
            heap_cp
        );
        assert_eq!(
            heap_extract_max(&mut heap),
            Some(1),
            "input: {:?}, heap: {:?}",
            cp,
            heap_cp
        );
        assert_eq!(heap_extract_max(&mut heap), None);
    }
}
