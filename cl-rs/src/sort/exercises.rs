use std::cmp::Ordering;

// 练习2.3-4，递归插入排序
pub fn insertion_sort_recursive<T: Ord>(v: &mut [T]) {
    let n = v.len();
    if n <= 1 {
        return;
    }
    insertion_sort_recursive0(v, 1);
}

// 练习2.3-6, 插入排序使用二分搜索
// 不写了，复杂度没变，还是要一个一个挪

// 思考题2-4，逆序对
pub fn reverse_pairs(xs: &Vec<i32>) -> i32 {
    let mut xms = xs.clone();
    let len = xs.len();
    let mut count = 0;
    divide_conquer_count(&mut xms, 0, len - 1, &mut count);
    count
}

fn divide_conquer_count(xms: &mut [i32], lhs: usize, rhs: usize, count: &mut i32) {
    if lhs < rhs {
        let mid = (lhs + rhs) / 2;
        divide_conquer_count(xms, lhs, mid, count);
        divide_conquer_count(xms, mid + 1, rhs, count);
        merge_count(xms, lhs, mid, rhs, count);
    }
}

fn merge_count(xms: &mut [i32], lhs: usize, mid: usize, rhs: usize, count: &mut i32) {
    let mut src = vec![];
    src.extend_from_slice(&xms[lhs..=rhs]);
    let (mut p, mut q) = (lhs, mid + 1);
    let mut idx = lhs;
    while p <= mid || q <= rhs {
        if p > mid {
            xms[idx] = src[q - lhs];
            q += 1;
        } else if q > rhs {
            xms[idx] = src[p - lhs];
            p += 1;
        } else if src[p - lhs].cmp(&src[q - lhs]) == Ordering::Less {
            xms[idx] = src[p - lhs];
            *count += (q - mid - 1) as i32;
            p += 1;
        } else {
            xms[idx] = src[q - lhs];
            if q == rhs {
                *count += ((mid - p + 1) * (rhs - mid)) as i32;
            }
            q += 1;
        }
        idx += 1;
    }
}

fn insertion_sort_recursive0<T: Ord>(v: &mut [T], rhs: usize) {
    if rhs == v.len() {
        return;
    }
    let mut j = rhs - 1;
    while v[j].cmp(&v[j + 1]) == Ordering::Greater {
        v.swap(j, j + 1);
        if j == 0 {
            break;
        }
        j -= 1;
    }
    insertion_sort_recursive0(v, rhs + 1);
}

#[cfg(test)]
mod tests {
    use super::insertion_sort_recursive;
    use super::reverse_pairs;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn merge_sort_recursive_works_empty() {
        let mut xs: Vec<i32> = vec![];
        insertion_sort_recursive(&mut xs);
        assert_eq!(xs, vec![]);
    }

    #[test]
    fn merge_sort_recursive_works_one_elem() {
        let mut xs1 = vec![1];
        insertion_sort_recursive(&mut xs1);
        assert_eq!(xs1, vec![1]);
    }

    #[test]
    fn merge_sort_recursive_works_two_distinct() {
        let mut xs2 = vec![1, -1];
        insertion_sort_recursive(&mut xs2);
        assert_eq!(xs2, vec![-1, 1]);
    }

    #[test]
    fn merge_sort_recursive_works_with_dup() {
        let mut xs3 = vec![1, 1, -1, 1, -1, -1];
        insertion_sort_recursive(&mut xs3);
        assert_eq!(xs3, vec![-1, -1, -1, 1, 1, 1]);
    }

    #[test]
    fn merge_sort_recursive_works() {
        let mut xs4 = vec![1, 1, -1, 6, -6, 5, 2];
        insertion_sort_recursive(&mut xs4);
        assert_eq!(xs4, vec![-6, -1, 1, 1, 2, 5, 6]);
    }

    #[test]
    fn reverse_pairs_works() {
        let xs = vec![2, 3, 8, 6, 1];
        let count = reverse_pairs(&xs);
        assert_eq!(count, 5);
    }

    #[test]
    fn reverse_pairs_works_100() {
        let mut xs = vec![1, 2, 3, 4, 5, 6];
        for _ in 0..100 {
            xs.shuffle(&mut thread_rng());
            let count_n2 = reverse_pair_naive(&xs);
            let count_divide_conquer = reverse_pairs(&mut xs);
            assert_eq!(count_n2, count_divide_conquer);
        }
    }

    fn reverse_pair_naive(xs: &Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..xs.len() {
            for j in 0..i {
                if xs[j] > xs[i] {
                    count += 1;
                }
            }
        }
        count
    }
}
