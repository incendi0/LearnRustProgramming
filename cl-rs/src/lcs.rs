pub fn longest_common_sequence(xs: &Vec<i32>, ys: &Vec<i32>) -> (Vec<Vec<i32>>, Vec<Vec<usize>>) {
    let (m, n) = (xs.len(), ys.len());
    let mut b = vec![vec![0; n + 1]; m + 1];
    let mut c = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if xs[i - 1] == ys[j - 1] {
                c[i][j] = c[i - 1][j - 1] + 1;
                b[i][j] = 0; // 0 for xs[i-1] and ys[j-1]
            } else if c[i - 1][j] >= c[i][j - 1] {
                c[i][j] = c[i - 1][j];
                b[i][j] = 1; // 1 for xs[i-1]
            } else {
                c[i][j] = c[i][j - 1];
                b[i][j] = 2; // 1 for ys[j-1]
            }
        }
    }
    (c, b)
}

pub fn print_lcs(b: &Vec<Vec<usize>>, xs: &Vec<i32>, xs_idx: usize, ys_idx: usize) {
    if xs_idx == 0 || ys_idx == 0 {
        return;
    }
    if b[xs_idx][ys_idx] == 0 {
        print_lcs(b, xs, xs_idx - 1, ys_idx - 1);
        print!("{}", xs[xs_idx - 1]);
    } else if b[xs_idx][ys_idx] == 1 {
        print_lcs(b, xs, xs_idx - 1, ys_idx);
    } else {
        print_lcs(b, xs, xs_idx, ys_idx - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_common_sequence_works() {
        let xs = vec![1, 2, 3, 2, 4, 1, 2];
        let ys = vec![2, 4, 3, 1, 2, 1];
        let (_c, b) = longest_common_sequence(&xs, &ys);
        print_lcs(&b, &xs, xs.len(), ys.len())
    }
}
