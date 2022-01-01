pub fn find_maximum_subarray_linear(xs: &Vec<i32>) -> (usize, usize, i32) {
    let n = xs.len();
    let (mut start, mut end) = (0, 0);
    let (mut max_start, mut max_end) = (0, 0);
    let mut max_so_far = xs[0];
    let mut max_sum = xs[0];
    for i in 1..n {
        if max_so_far < 0 {
            start = i;
            end = i;
            max_so_far = xs[i];
        } else {
            max_so_far += xs[i];
            end += 1;
        }
        if max_sum < max_so_far {
            max_sum = max_so_far;
            max_start = start;
            max_end = end;
        }
    }
    (max_start, max_end, max_sum)
}

pub fn find_maximum_subarray(xs: &Vec<i32>) -> i32 {
    find_maximum_subarray_rec(xs, 0, xs.len() - 1).2
}

fn find_maximum_subarray_rec(xs: &Vec<i32>, lhs: usize, rhs: usize) -> (usize, usize, i32) {
    if lhs == rhs {
        (lhs, rhs, xs[lhs])
    } else {
        let mid = (lhs + rhs) / 2;
        let (left_low, left_high, left_sum) = find_maximum_subarray_rec(xs, lhs, mid);
        let (right_low, right_high, right_sum) = find_maximum_subarray_rec(xs, mid + 1, rhs);
        let (cross_low, cross_high, cross_sum) = find_maximum_cross_subarray(xs, lhs, mid, rhs);
        if left_sum >= right_sum && left_sum >= cross_sum {
            (left_low, left_high, left_sum)
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            (right_low, right_high, right_sum)
        } else {
            (cross_low, cross_high, cross_sum)
        }
    }
}

fn find_maximum_cross_subarray(
    xs: &Vec<i32>,
    lhs: usize,
    mid: usize,
    rhs: usize,
) -> (usize, usize, i32) {
    let mut left_sum = i32::MIN;
    let mut sum = 0;
    let mut max_left = mid;
    for i in (lhs..=mid).rev() {
        sum += xs[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }
    let mut right_sum = i32::MIN;
    sum = 0;
    let mut max_right = mid + 1;
    for i in mid + 1..=rhs {
        sum += xs[i];
        if sum > right_sum {
            right_sum = sum;
            max_right = i;
        }
    }
    (max_left, max_right, left_sum + right_sum)
    // 练习4.1-4，如果结果为空，和为0，可以修改如下
    // if left_sum + right_sum >= 0 {
    //     (max_left, max_right, left_sum + right_sum)
    // } else {
    //     (xs.len(), xs.len(), 0)
    // }
}

#[cfg(test)]
mod tests {
    use super::find_maximum_subarray;
    use super::find_maximum_subarray_linear;

    #[test]
    fn find_maximum_subarray_works() {
        let mut xs: Vec<i32> = vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        assert_eq!(find_maximum_subarray(&mut xs), 43);
    }

    #[test]
    fn find_maximum_subarray_works_all_negative_return_max() {
        let mut xs: Vec<i32> = vec![
            -13, -3, -25, -20, -3, -16, -23, -18, -20, -7, -12, -5, -22, -15, -4, -7,
        ];
        assert_eq!(find_maximum_subarray(&mut xs), -3);
    }

    #[test]
    fn find_maximum_subarray_works_linear() {
        let mut xs: Vec<i32> = vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        assert_eq!(find_maximum_subarray_linear(&mut xs), (7, 10, 43));
    }

    #[test]
    fn find_maximum_subarray_works_linear_all_negative_return_max() {
        let mut xs: Vec<i32> = vec![
            -13, -3, -25, -20, -3, -16, -23, -18, -20, -7, -12, -5, -22, -15, -4, -7, -8,
        ];
        assert_eq!(find_maximum_subarray_linear(&mut xs).2, -3);
    }
}
