use std::cmp::Ordering;

// 2.1 插入排序
pub fn insertion_sort<T: Ord>(v: &mut [T]) {
    let n = v.len();
    for i in 1..n {
        let mut j = i - 1;
        while v[j].cmp(&v[j + 1]) == Ordering::Greater {
            v.swap(j, j + 1);
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    #[test]
    fn insertion_sort_works_empty() {
        let mut xs: Vec<i32> = vec![];
        insertion_sort(&mut xs);
        assert_eq!(xs, vec![]);
    }

    #[test]
    fn insertion_sort_works_one_elem() {
        let mut xs1 = vec![1];
        insertion_sort(&mut xs1);
        assert_eq!(xs1, vec![1]);
    }

    #[test]
    fn insertion_sort_works_two_distinct() {
        let mut xs2 = vec![1, -1];
        insertion_sort(&mut xs2);
        assert_eq!(xs2, vec![-1, 1]);
    }

    #[test]
    fn insertion_sort_works_with_dup() {
        let mut xs3 = vec![1, 1, -1, 1, -1, -1];
        insertion_sort(&mut xs3);
        assert_eq!(xs3, vec![-1, -1, -1, 1, 1, 1]);
    }

    #[test]
    fn insertion_sort_works() {
        let mut xs4 = vec![1, 1, -1, 6, -6, 5, 2];
        insertion_sort(&mut xs4);
        assert_eq!(xs4, vec![-6, -1, 1, 1, 2, 5, 6]);
    }
}
