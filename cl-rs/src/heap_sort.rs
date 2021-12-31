use std::fmt::Debug;

pub fn heap_sort<T: Ord + Debug>(xs: &mut [T]) {
    if xs.len() <= 1 {
        return;
    }
    build_max_heap(xs);
    for i in (1..=xs.len() - 1).rev() {
        xs.swap(0, i);
        max_heapify(xs, 0, i);
    }
}

pub fn build_max_heap<T: Ord>(xs: &mut [T]) {
    // 从叶子节点倒序维护堆
    for i in (0..=parent(xs.len() - 1)).rev() {
        max_heapify(xs, i, xs.len());
    }
}

pub fn parent(i: usize) -> usize {
    (i - 1) / 2
}

pub fn left(i: usize) -> usize {
    i * 2 + 1
}

pub fn right(i: usize) -> usize {
    i * 2 + 2
}

pub fn max_heapify<T: Ord>(xs: &mut [T], i: usize, limit: usize) {
    let (l, r) = (left(i), right(i));
    let mut largest;
    if l < limit && xs[l] > xs[i] {
        largest = l;
    } else {
        largest = i;
    }
    if r < limit && xs[r] > xs[largest] {
        largest = r;
    }
    if largest != i {
        xs.swap(largest, i);
        max_heapify(xs, largest, limit);
    }
}

// 练习6.2-5 递归改迭代
fn max_heapify_iterative<T: Ord>(xs: &mut [T], i: usize, limit: usize) {
    let mut idx = i;
    let mut largest = find_largest(xs, idx, limit);
    while largest != idx {
        xs.swap(largest, idx);
        idx = largest;
        largest = find_largest(xs, idx, limit);
    }
}

fn find_largest<T: Ord>(xs: &mut [T], i: usize, limit: usize) -> usize {
    let (l, r) = (left(i), right(i));
    let mut largest = i;
    if l < limit && xs[l] > xs[largest] {
        largest = l;
    }
    if r < limit && xs[r] > xs[largest] {
        largest = r;
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::heap_sort;
    use super::max_heapify;
    use super::max_heapify_iterative;

    #[test]
    fn max_heapify_works1() {
        let mut xs: Vec<i32> = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        let n = xs.len();
        max_heapify(&mut xs, 1, n);
        assert_eq!(xs, vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
    }

    #[test]
    fn max_heapify_iterative_works1() {
        let mut xs: Vec<i32> = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        let n = xs.len();
        max_heapify_iterative(&mut xs, 1, n);
        assert_eq!(xs, vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
    }

    #[test]
    fn heap_sort_works_empty() {
        let mut xs: Vec<i32> = vec![];
        heap_sort(&mut xs);
        assert_eq!(xs, vec![]);
    }

    #[test]
    fn heap_sort_works_one_elem() {
        let mut xs1 = vec![1];
        heap_sort(&mut xs1);
        assert_eq!(xs1, vec![1]);
    }

    #[test]
    fn heap_sort_works_two_distinct() {
        let mut xs2 = vec![1, -1];
        heap_sort(&mut xs2);
        assert_eq!(xs2, vec![-1, 1]);
    }

    #[test]
    fn heap_sort_works_with_dup() {
        let mut xs3 = vec![1, 1, -1, 1, -1, -1];
        heap_sort(&mut xs3);
        assert_eq!(xs3, vec![-1, -1, -1, 1, 1, 1]);
    }

    #[test]
    fn heap_sort_works() {
        let mut xs4 = vec![1, 1, -1, 6, -6, 5, 2];
        heap_sort(&mut xs4);
        assert_eq!(xs4, vec![-6, -1, 1, 1, 2, 5, 6]);
    }
}
