use std::cmp::max;

// 15.4-5 LIS
pub fn longest_increasing_sequence(xs: &Vec<i32>) -> (Vec<i32>, Vec<usize>) {
    let m = xs.len();
    let mut f = vec![1; m];
    let mut g = vec![0; m];
    for i in 0..m {
        g[i] = i;
    }
    for i in 1..m {
        for j in 0..i {
            if xs[i] > xs[j] && f[i] < f[j] + 1 {
                f[i] = f[j] + 1;
                g[i] = j;
            }
        }
    }
    (f, g)
}

pub fn print_lis(xs: &Vec<i32>, lis: &Vec<i32>, pvs: &Vec<usize>) {
    let (mut longest, mut idx) = (1, 0);
    for i in 1..xs.len() {
        if lis[i] > longest {
            longest = lis[i];
            idx = i;
        }
    }
    print_lis_aux(xs, pvs, idx);
}

fn print_lis_aux(xs: &Vec<i32>, pvs: &Vec<usize>, idx: usize) {
    print!("{}", xs[idx]);
    if pvs[idx] != idx {
        print_lis_aux(xs, pvs, pvs[idx]);
    }
}

// 15.4-6 LIS nlgn
// https://leetcode-cn.com/problems/longest-increasing-subsequence/solution/zui-chang-shang-sheng-zi-xu-lie-by-leetcode-soluti/
pub fn longest_increasing_sequence_nlgn(xs: &Vec<i32>) -> i32 {
    let mut stack = vec![];
    for &x in xs {
        if stack.is_empty() || x > *stack.last().unwrap() {
            stack.push(x);
        } else {
            let (mut l, mut r) = (0, stack.len() - 1);
            while l < r {
                let mid = (l + r) / 2;
                if stack[mid] > x {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            stack[l] = x;
        }
    }
    println!("LIS of {:?} is {:?}", xs, stack);
    stack.len() as i32
}

// 16.2-2 knapsack
// https://oi-wiki.org/dp/knapsack/
// 0-1背包
// 已知条件有第i个物品的重量wi，价值vi，以及背包的总容量W。
// f(i, j) 表示选择前i个物品，使用的容量为j，能获取到的最大价值
// f(i, j) = max(f(i - 1, j), f(i - 1, j - w[i]) + v[i])
// simplified to
// f(j) = max(f(j), f(j - w[i]) + v[i])
pub fn knapsack01(weights: &Vec<usize>, values: &Vec<usize>, w: usize) -> usize {
    let n = weights.len();
    let mut f = vec![vec![0; w + 1]; n + 1];
    for i in 1..=n {
        for j in 0..=w {
            f[i][j] = f[i - 1][j];
            if j >= weights[i - 1] {
                f[i][j] = max(f[i][j], f[i - 1][j - weights[i - 1]] + values[i - 1]);
            }
        }
    }
    f[n][w]
}
pub fn knapsack01_simplified(weights: &Vec<usize>, values: &Vec<usize>, w: usize) -> usize {
    let n = weights.len();
    let mut f = vec![0; w + 1];
    for i in 1..=n {
        for j in (weights[i - 1]..=w).rev() {
            f[j] = max(f[j], f[j - weights[i - 1]] + values[i - 1]);
        }
    }
    f[w]
}
// 完全背包
pub fn knapsack01_complete(weights: &Vec<usize>, values: &Vec<usize>, w: usize) -> usize {
    let n = weights.len();
    let mut f = vec![0; w + 1];
    for i in 1..=n {
        // 正向循环，第i个物品可以多次放
        // 反向循环，f[j]先于f[j-w[i]]被计算出来，是0-1背包
        for j in weights[i - 1]..=w {
            f[j] = max(f[j], f[j - weights[i - 1]] + values[i - 1]);
        }
    }
    f[w]
}
// 多重背包
pub fn knapsack01_multiple(
    weights: &Vec<usize>,
    values: &Vec<usize>,
    numbers: &Vec<usize>,
    w: usize,
) -> usize {
    let n = weights.len();
    let mut f = vec![vec![0; w + 1]; n + 1];
    for i in 1..=n {
        for j in 0..=w {
            let mut k = 0;
            while k <= numbers[i - 1] && k * weights[i - 1] <= j {
                f[i][j] = max(
                    f[i][j],
                    f[i - 1][j - weights[i - 1] * k] + values[i - 1] * k,
                );
                k += 1;
            }
        }
    }
    f[n][w]
}
// 分组背包
// 有N件物品和一个容量为w的背包。第i件物品的体积是weights[i]，价值是values[i]。这些物品被划分为若干组，每组中的物品互相冲突，最多选一件。
// 求解将哪些物品装入背包可使这些物品的费用总和不超过背包容量，且价值总和最大。
// 这个问题变成了每组物品有若干种策略：是选择本组的某一件，还是一件都不选。也就是说设f[i][j]表示前i组物品花费费用j能取得的最大权值，
// 则有f[i][j] = max(f[i - 1][j], f[i - 1][w - weights[i]] + values[i] for i in group)。

// 使用一维数组的伪代码如下：

// for所有的组k
//     for所有的i属于组k
//         for v=V..0
//             f[v]=max{f[v],f[v-weights[i]]+values[i]}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_increasing_sequence_works() {
        let xs = vec![5, 3, 4, 2, 7, 9, 1, 8, 6];
        let (f, g) = longest_increasing_sequence(&xs);
        print_lis(&xs, &f, &g);
    }

    #[test]
    fn longest_increasing_sequence_nlgn_works() {
        let xs = vec![5, 3, 4, 2, 7, 9, 1, 8, 6];
        let longest = longest_increasing_sequence_nlgn(&xs);
        assert_eq!(longest, 4);
    }
}
