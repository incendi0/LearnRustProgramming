pub fn quick_sort<T: Ord + Clone>(xs: &mut [T]) {
    let n = xs.len();
    if n <= 1 {
        return;
    }
    quick_sort0(xs, 0, n - 1);
}

fn quick_sort0<T: Ord + Clone>(xs: &mut [T], lhs: usize, rhs: usize) {
    if lhs < rhs {
        let mid = partition(xs, lhs, rhs);
        // caution if mid == 0, underflow
        if mid >= 1 {
            quick_sort0(xs, lhs, mid - 1);
        }
        quick_sort0(xs, mid + 1, rhs);
    }
}

pub fn partition<T: Ord + Clone>(xs: &mut [T], lhs: usize, rhs: usize) -> usize {
    let pivot = xs[rhs].clone();
    let mut i = lhs; // exclusive
    for j in lhs..rhs {
        if xs[j] <= pivot {
            xs.swap(i, j);
            i += 1;
        }
    }
    xs.swap(i, rhs);
    i
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn quick_sort_works_empty() {
        let mut xs: Vec<i32> = vec![];
        quick_sort(&mut xs);
        assert_eq!(xs, vec![]);
    }

    #[test]
    fn quick_sort_works_one_elem() {
        let mut xs1 = vec![1];
        quick_sort(&mut xs1);
        assert_eq!(xs1, vec![1]);
    }

    #[test]
    fn quick_sort_works_two_distinct() {
        let mut xs2 = vec![1, -1];
        quick_sort(&mut xs2);
        assert_eq!(xs2, vec![-1, 1]);
    }

    #[test]
    fn quick_sort_works_with_dup() {
        let mut xs3 = vec![1, 1, -1, 1, -1, -1];
        quick_sort(&mut xs3);
        assert_eq!(xs3, vec![-1, -1, -1, 1, 1, 1]);
    }

    #[test]
    fn quick_sort_works() {
        let mut xs4 = vec![1, 1, -1, 6, -6, 5, 2];
        quick_sort(&mut xs4);
        assert_eq!(xs4, vec![-6, -1, 1, 1, 2, 5, 6]);
    }
}
