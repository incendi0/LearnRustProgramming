use std::cmp::max;

struct Solution {}

impl Solution {
    /// 记录多出的`(`的数目，涉及到*，记录一个最大值hi，一个最小值lo
    /// 如果遍历完之后，最小值为0，则合法
    /// 1. 如果遇到`(`，hi++，lo++
    /// 2. 如果遇到`)`，如果hi == 0，则非法
    /// 3. 如果遇到`*`，则hi++
    pub fn check_valid_string(s: String) -> bool {
        let (mut lo, mut hi) = (0, 0);
        let xs = s.as_bytes();
        for &ch in xs {
            if ch == b'(' {
                lo += 1;
                hi += 1;
            } else if ch == b')' {
                lo = max(lo - 1, 0);
                if hi == 0 {
                    return false;
                }
                hi -= 1;
            } else {
                lo = max(lo - 1, 0);
                hi += 1;
            }
        }
        lo == 0
    }

    /// f(i, j)表示前i个字符，有j个未匹配的`(`
    /// 如果第i个字符是`(`，则f(i, j) = f(i - 1, j - 1)
    /// 如果第i个字符是`)`，则f(i, j) = f(i - 1, j + 1)
    /// 如果第i个字符是`*`，则f(i, j) |= f(i - 1, j) || f(i - 1, j - 1) || f(i - 1, j + 1)
    pub fn check_valid_string_0(s: String) -> bool {
        let xs = s.as_bytes();
        let n = xs.len();
        let mut f = vec![vec![false; n + 1]; n + 1];
        f[0][0] = true;
        for i in 1..n + 1 {
            for j in 0..i + 1 {
                if xs[i - 1] == b'(' {
                    if j >= 1 {
                        f[i][j] |= f[i - 1][j - 1];
                    }
                } else if xs[i - 1] == b')' {
                    if j < i {
                        f[i][j] |= f[i - 1][j + 1];
                    }
                } else {
                    f[i][j] |= f[i - 1][j];
                    if j >= 1 {
                        f[i][j] |= f[i - 1][j - 1];
                    }
                    if j < i {
                        f[i][j] |= f[i - 1][j + 1];
                    }
                }
            }
        }
        f[n][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_valid_parenthesis_0() {
        assert!(Solution::check_valid_string_0("()(*)".to_string()));
        assert!(Solution::check_valid_string_0("()".to_string()));
        assert!(Solution::check_valid_string_0("(*)".to_string()));
        assert!(Solution::check_valid_string_0("(*))".to_string()));
    }

    #[test]
    fn test_valid_parenthesis() {
        assert!(Solution::check_valid_string("()(*)".to_string()));
        assert!(Solution::check_valid_string("()".to_string()));
        assert!(Solution::check_valid_string("(*)".to_string()));
        assert!(Solution::check_valid_string("(*))".to_string()));
    }
}
