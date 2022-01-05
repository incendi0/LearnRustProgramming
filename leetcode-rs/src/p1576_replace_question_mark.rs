struct Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut ss: Vec<char> = s.chars().collect();
        let mut prev;
        let mut next;
        let n = ss.len();
        for i in 0..n {
            if ss[i] != '?' {
                continue;
            }
            prev = if i > 0 { Some(ss[i - 1]) } else { None };
            next = if i < n - 1 { Some(ss[i + 1]) } else { None };
            match (prev, next) {
                (Some(p), Some(n)) => {
                    ss[i] = Self::find_char_not2(p, n);
                }
                (Some(p), None) => {
                    ss[i] = Self::find_char_not(p);
                }
                (None, Some(n)) => {
                    ss[i] = Self::find_char_not(n);
                }
                _ => {
                    ss[i] = 'a';
                }
            }
        }
        ss.iter().collect::<String>()
    }

    fn find_char_not(t: char) -> char {
        for ch in 'a'..='z' {
            if ch != t {
                return ch;
            }
        }
        '?'
    }
    fn find_char_not2(t: char, t2: char) -> char {
        for ch in 'a'..='z' {
            if ch != t && ch != t2 {
                return ch;
            }
        }
        '?'
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        println!("{:?}", Solution::modify_string("?zs".to_string()));
        println!("{:?}", Solution::modify_string("ubv?w".to_string()));
        println!("{:?}", Solution::modify_string("j?qg??b".to_string()));
        println!("{:?}", Solution::modify_string("??yw?ipkj?".to_string()));
        println!("{:?}", Solution::modify_string("???????".to_string()));
    }
}
