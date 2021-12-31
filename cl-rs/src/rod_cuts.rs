use std::cmp::max;
use std::collections::HashMap;

pub fn memoized_cut_prod(prices: &HashMap<i32, i32>, n: i32) -> i32 {
    let mut cache = HashMap::new();
    memoized_cut_prod_aux(prices, n, &mut cache)
}

fn memoized_cut_prod_aux(prices: &HashMap<i32, i32>, n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if cache.contains_key(&n) {
        return *cache.get(&n).unwrap();
    }
    let mut max_price_sum = 0;
    for len in 1..=n {
        if prices.contains_key(&len) {
            max_price_sum = max(
                max_price_sum,
                prices.get(&len).unwrap() + memoized_cut_prod_aux(prices, n - len, cache),
            );
        }
    }
    cache.insert(n, max_price_sum);
    max_price_sum
}

fn bottom_up_cut_prod(prices: &HashMap<i32, i32>, n: i32) -> (Vec<i32>, Vec<i32>) {
    let mut dp = vec![0; (n + 1) as usize];
    let mut first_cuts = vec![0; (n + 1) as usize];
    for j in 1..=n {
        let mut max_price_sum = i32::MIN;
        for i in 1..=j {
            if prices.contains_key(&i) {
                if max_price_sum < *prices.get(&i).unwrap() + dp[(j - i) as usize] {
                    max_price_sum = *prices.get(&i).unwrap() + dp[(j - i) as usize];
                    first_cuts[j as usize] = i;
                }
            }
        }
        dp[j as usize] = max_price_sum
    }
    (dp, first_cuts)
}

pub fn print_cut_rod_solution(prices: &HashMap<i32, i32>, n: i32) -> Vec<i32> {
    let (_, first_cuts) = bottom_up_cut_prod(prices, n);
    let mut solution = vec![];
    let mut n = n;
    while n > 0 {
        solution.push(first_cuts[n as usize]);
        n -= first_cuts[n as usize];
    }
    solution
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn memoized_cut_prod_works() {
        let xs = HashMap::from([
            (1, 1),
            (2, 5),
            (3, 8),
            (4, 9),
            (5, 10),
            (6, 17),
            (7, 17),
            (8, 20),
            (9, 24),
            (10, 30),
        ]);
        let best_cuts = vec![1, 5, 8, 10, 13, 17, 18, 22, 25, 30];
        for i in 1..=10 {
            let r = memoized_cut_prod(&xs, i);
            assert_eq!(r, best_cuts[(i - 1) as usize]);
        }
        // for i in 11..=30 {
        //     println!("{} can gain {}", i, memoized_cut_prod(&xs, i));
        // }
    }

    #[test]
    fn bottom_up_cut_prod_works() {
        let xs = HashMap::from([
            (1, 1),
            (2, 5),
            (3, 8),
            (4, 9),
            (5, 10),
            (6, 17),
            (7, 17),
            (8, 20),
            (9, 24),
            (10, 30),
        ]);
        // let best_cuts = vec![1, 5, 8, 10, 13, 17, 18, 22, 25, 30];
        for i in 1..=10 {
            let _r = print_cut_rod_solution(&xs, i);
            // println!("{:?}", r);
            // assert_eq!(r, best_cuts[(i - 1) as usize]);
        }
        // for i in 11..=30 {
        //     println!("{} can gain {}", i, memoized_cut_prod(&xs, i));
        // }
    }
}
