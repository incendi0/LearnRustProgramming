struct Solution;

/// 记忆化搜索
/// tricky part是搜索的终点
/// 如果当前猫鼠的位置以及走的顺序出现过，则是平局，那么状态可以定为(搜索顺序，mouse坐标，cat坐标)
/// 最多有 n * n * 2 个搜索顺序（事实上2n就足够了，2n步老鼠到不了洞口，再循环往复也没办法了，anyway，都能ac
/// 整个搜索空间为 n^4
/// 题目给的数据范围在50以内，
impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut memo = vec![vec![vec![-1; n]; n]; n * n * 2];
        Self::dfs(&graph, &mut memo, 0, 1, 2)
    }

    fn dfs(
        graph: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<Vec<i32>>>,
        t: usize,
        mouse: usize,
        cat: usize,
    ) -> i32 {
        if t == graph.len() * graph.len() * 2 {
            return 0;
        }
        if mouse == cat {
            memo[t][mouse][cat] = 2;
            return 2;
        }
        if mouse == 0 {
            memo[t][mouse][cat] = 1;
            return 1;
        }
        if memo[t][mouse][cat] != -1 {
            return memo[t][mouse][cat];
        }
        let turn = t % 2;
        let mut flag;
        if turn == 1 {
            // cat turn
            flag = true;
            for &v in graph[cat].iter() {
                if v != 0 {
                    let v = v as usize;
                    let result = Self::dfs(graph, memo, t + 1, mouse, v);
                    if result == 2 {
                        memo[t][mouse][cat] = 2;
                        return 2;
                    } else if result != 1 {
                        flag = false;
                    }
                }
            }
            if flag {
                // 所有的搜索都是 mouse 胜利
                memo[t][mouse][cat] = 1;
                return 1;
            } else {
                // 有平局
                memo[t][mouse][cat] = 0;
                return 0;
            }
        } else {
            // mouse turn
            flag = true;
            for &v in graph[mouse].iter() {
                let v = v as usize;
                let result = Self::dfs(graph, memo, t + 1, v, cat);
                if result == 1 {
                    memo[t][v][cat] = 1;
                    return 1;
                } else if result != 2 {
                    flag = false;
                }
            }
            if flag {
                // 所有的搜索都是 cat 胜利
                memo[t][mouse][cat] = 2;
                return 2;
            } else {
                // 有平局
                memo[t][mouse][cat] = 0;
                return 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cat_mouse_game_works() {
        let graph = vec![
            vec![2, 5],
            vec![3],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![2, 3],
            vec![0, 2, 3],
        ];
        assert_eq!(Solution::cat_mouse_game(graph), 0);
    }
}
