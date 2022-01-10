struct Solution;

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let xs = num.as_bytes().to_vec();
        let n = xs.len();
        for i in 1..n {
            for j in i + 1..n {
                if Self::helper(&xs, i, j) {
                    return true;
                }
            }
        }
        false
    }

    // [lhs, mid) [mid, rhs)
    fn helper(xs: &Vec<u8>, i: usize, j: usize) -> bool {
        let (mut lhs, mut mid, mut rhs) = (0, i, j);
        loop {
            let first = xs[lhs..mid].to_vec();
            if first[0] == b'0' && first.len() > 1 {
                return false;
            }
            let second = xs[mid..rhs].to_vec();
            if second[0] == b'0' && second.len() > 1 {
                return false;
            }
            let third = Self::add(first, second);
            if rhs + third.len() > xs.len() {
                return false;
            } else {
                if xs[rhs..rhs + third.len()] != third {
                    return false;
                } else {
                    if rhs + third.len() == xs.len() {
                        return true;
                    } else {
                        lhs = mid;
                        mid = rhs;
                        rhs = rhs + third.len();
                    }
                }
            }
        }
    }

    fn add(lhs: Vec<u8>, rhs: Vec<u8>) -> Vec<u8> {
        let (mut lhs, mut rhs) = (lhs, rhs);
        lhs.reverse();
        rhs.reverse();
        let (m, n) = (lhs.len(), rhs.len());
        let (mut i, mut j) = (0, 0);
        let mut ret = vec![];
        let mut carry = 0;
        while i < m || j < n {
            let a = if i < m { lhs[i] - b'0' } else { 0 };
            let b = if j < n { rhs[j] - b'0' } else { 0 };
            let sum = a + b + carry;
            ret.push(sum % 10 + b'0');
            carry = sum / 10;
            i += 1;
            j += 1;
        }
        if carry > 0 {
            ret.push(b'1');
        }
        ret.reverse();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert!(Solution::is_additive_number("112358".into()));
        assert!(Solution::is_additive_number("199100199".into()));
        assert!(!Solution::is_additive_number("1".into()));
        assert!(!Solution::is_additive_number("12".into()));
        assert!(Solution::is_additive_number("123".into()));
        assert!(!Solution::is_additive_number("1023".into()));
        assert!(Solution::is_additive_number("1235".into()));
    }

    #[test]
    fn add_works() {
        let lhs = vec![b'1', b'2', b'3'];
        let rhs = vec![b'9', b'8', b'9'];
        assert_eq!(Solution::add(lhs, rhs), vec![b'1', b'1', b'1', b'2']);
    }
}
