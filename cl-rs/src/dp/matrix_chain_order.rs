pub fn matrix_chain_order(p: &Vec<i32>) -> (Vec<Vec<i32>>, Vec<Vec<usize>>) {
    let n = p.len() - 1;
    let mut m = vec![vec![0; n + 1]; n + 1];
    let mut s = vec![vec![0; n + 1]; n + 1];
    for len in 2..=n {
        for i in 1..=n - len + 1 {
            let j = i + len - 1;
            m[i][j] = i32::MAX;
            for k in i..j {
                let q = m[i][k] + m[k + 1][j] + p[i - 1] * p[k] * p[j];
                if q < m[i][j] {
                    m[i][j] = q;
                    s[i][j] = k;
                }
            }
        }
    }
    (m, s)
}

pub fn print_optimal_parens(s: &Vec<Vec<usize>>, lhs: usize, rhs: usize) {
    if lhs == rhs {
        print!("A");
    } else {
        print!("(");
        print_optimal_parens(s, lhs, s[lhs][rhs]);
        print_optimal_parens(s, s[lhs][rhs] + 1, rhs);
        print!(")");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_chain_order_works() {
        let xs = vec![30, 35, 15, 5, 10, 20, 25];
        let (m, s) = matrix_chain_order(&xs);
        println!("m: {:?}", m);
        println!("s: {:?}", s);
        print_optimal_parens(&s, 1, xs.len() - 1);
    }
}
