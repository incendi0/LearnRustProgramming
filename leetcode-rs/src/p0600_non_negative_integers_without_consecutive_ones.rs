struct Solution {}

impl Solution {
    /// f(i)表示i位以0为最高位的符合要求的数字的数目（即不存在连续的1的数字）
    /// 二进制表示可以看做trie
    /// 假设g(depth, num)表示深度为depth，且当前数目为num的符合要求的数字的数目
    /// g(depth, 0) = g(depth - 1, 1) + g(depth - 1, 0)
    /// g(depth, 1) = g(depth - 1, 0)
    /// 上面两个式子结合 g(depth, 0) = g(depth - 2, 0) + g(depth - 1, 0)
    /// 则f(i) = f(i - 2) + f(i - 1)，初始值f(0) = f(1) = 1
    /// 对于一个32位有符号数字，如果不考虑符号位，则计算到f(31)就可以满足要求
    /// 从高位到低位枚举n的每位数字，同时标记上一位是否是prevBitIsone = false;
    /// 1. 如果当前位值为1，累加f(i + 1)，如果prevBitIsone = true，则直接返回结果，否则prevBitIsone = true
    /// 2. 如果当前位值为0，prevBitIsone = false
    /// 3. 返回结果+1
    pub fn find_integers(n: i32) -> i32 {
        const N: usize = 31;
        let mut f = [1; N];
        for i in 2..N {
            f[i] = f[i - 1] + f[i - 2];
        }
        let mut prev_bit_is_one = false;
        let mut ret = 0;
        for i in (0..N - 1).rev() {
            if (n & 1 << i) != 0 {
                ret += f[i + 1];
                if prev_bit_is_one {
                    return ret;
                }
                prev_bit_is_one = true;
            } else {
                prev_bit_is_one = false;
            }
        }
        ret + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_integers() {
        assert_eq!(Solution::find_integers(5), 5);
    }
}
