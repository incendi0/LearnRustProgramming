/**
给你一个整数 n，请你帮忙统计一下我们可以按下述规则形成多少个长度为 n 的字符串：

    字符串中的每个字符都应当是小写元音字母（'a', 'e', 'i', 'o', 'u'）
    每个元音 'a' 后面都只能跟着 'e'
    每个元音 'e' 后面只能跟着 'a' 或者是 'i'
    每个元音 'i' 后面 不能 再跟着另一个 'i'
    每个元音 'o' 后面只能跟着 'i' 或者是 'u'
    每个元音 'u' 后面只能跟着 'a'

由于答案可能会很大，所以请你返回 模 10^9 + 7 之后的结果。

 

示例 1：

输入：n = 1
输出：5
解释：所有可能的字符串分别是："a", "e", "i" , "o" 和 "u"。

示例 2：

输入：n = 2
输出：10
解释：所有可能的字符串分别是："ae", "ea", "ei", "ia", "ie", "io", "iu", "oi", "ou" 和 "ua"。

示例 3：

输入：n = 5
输出：68

 

提示：

    1 <= n <= 2 * 10^4

 */
struct Solution;
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        // f(i, j) 表示前i个字符以j结尾的个数
        // j是0..=4，对应a,e,i,o,u
        // 转移关系如下
        // aeiou a
        // x  x
        // aeiou e
        //  x xx 
        // aeiou i
        // x x x
        // aeiou o
        // xx xx
        // aeiou u
        // xx  x
        const N: i32 = 1000000007;
        let mut f = vec![vec![1; 5]; n as usize];
        for i in 1..n as usize {
            f[i][0] = ((f[i - 1][1] + f[i - 1][2]) % N + f[i - 1][4]) % N;
            f[i][1] = (f[i - 1][0] + f[i - 1][2]) % N;
            f[i][2] = (f[i - 1][1] + f[i - 1][3]) % N;
            f[i][3] = f[i - 1][2];
            f[i][4] = (f[i - 1][2] + f[i - 1][3]) % N;
        }
        let mut ret = 0;
        for i in 0..5 {
            ret = (ret + f[(n - 1) as usize][i]) % N;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_vowel_permutation(1), 5);
        assert_eq!(Solution::count_vowel_permutation(2), 10);
        assert_eq!(Solution::count_vowel_permutation(5), 68);
    }
}
