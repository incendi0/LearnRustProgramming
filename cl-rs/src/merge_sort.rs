use std::cmp::Ordering;

// 2.3.1 分治法
pub fn merge_sort<T: Ord + Clone>(xs: &mut [T]) {
    if xs.len() <= 1 {
        return;
    }
    merge_sort_with_idx(xs, 0, xs.len() - 1);
}

fn merge_sort_with_idx<T: Ord + Clone>(xs: &mut [T], lhs: usize, rhs: usize) {
    if lhs < rhs {
        let mid = (lhs + rhs) / 2;
        merge_sort_with_idx(xs, lhs, mid);
        merge_sort_with_idx(xs, mid + 1, rhs);
        merge(xs, lhs, mid, rhs);
    }
}

fn merge<T: Ord + Clone>(xs: &mut [T], lhs: usize, mid: usize, rhs: usize) {
    let mut src = vec![];
    src.extend_from_slice(&xs[lhs..=rhs]);
    let (mut p, mut q) = (lhs, mid + 1);
    let mut idx = lhs;
    while p <= mid || q <= rhs {
        if p > mid {
            xs[idx] = src[q - lhs].clone();
            q += 1;
        } else if q > rhs {
            xs[idx] = src[p - lhs].clone();
            p += 1;
        } else if src[p - lhs].cmp(&src[q - lhs]) == Ordering::Less {
            xs[idx] = src[p - lhs].clone();
            p += 1;
        } else {
            xs[idx] = src[q - lhs].clone();
            q += 1;
        }
        idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn merge_sort_works_empty() {
        let mut xs: Vec<i32> = vec![];
        merge_sort(&mut xs);
        assert_eq!(xs, vec![]);
    }

    #[test]
    fn merge_sort_works_one_elem() {
        let mut xs1 = vec![1];
        merge_sort(&mut xs1);
        assert_eq!(xs1, vec![1]);
    }

    #[test]
    fn merge_sort_works_two_distinct() {
        let mut xs2 = vec![1, -1];
        merge_sort(&mut xs2);
        assert_eq!(xs2, vec![-1, 1]);
    }

    #[test]
    fn merge_sort_works_with_dup() {
        let mut xs3 = vec![1, 1, -1, 1, -1, -1];
        merge_sort(&mut xs3);
        assert_eq!(xs3, vec![-1, -1, -1, 1, 1, 1]);
    }

    #[test]
    fn merge_sort_works() {
        let mut xs4 = vec![1, 1, -1, 6, -6, 5, 2];
        merge_sort(&mut xs4);
        assert_eq!(xs4, vec![-6, -1, 1, 1, 2, 5, 6]);
    }
}
