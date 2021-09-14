struct Solution {}

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let (mut ret, mut len) = (&"".to_string(), 0);
        for seq in dictionary.iter() {
            if Self::is_subseq(&s, seq) {
                if seq.len() > len {
                    ret = &seq;
                    len = seq.len();
                } else if seq.len() == len && seq.le(ret) {
                    ret = seq;
                }
            }
        }
        ret.clone()
    }

    fn is_subseq(s: &str, seq: &str) -> bool {
        let ss: Vec<char> = s.chars().collect();
        let mut idx = 0;
        for ch in seq.chars() {
            while idx < ss.len() && ss[idx] != ch {
                idx += 1;
            }
            if idx == ss.len() {
                return false;
            } else {
                idx += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_subseq() {
        assert!(Solution::is_subseq("()(*)", "(*)"));
        assert!(Solution::is_subseq("()(*)", "()(*)"));
        assert!(Solution::is_subseq("()", ""));
        assert!(!Solution::is_subseq("(*)", "()*"));
        assert!(!Solution::is_subseq("(*))", "(*)))"));
    }

    #[test]
    fn test_find_longest_word() {
        assert_eq!(
            Solution::find_longest_word(
                "abpcplea".to_string(),
                vec![
                    "ale".to_string(),
                    "apple".to_string(),
                    "monkey".to_string(),
                    "plea".to_string()
                ]
            ),
            "apple".to_string()
        );
        assert_eq!(
            Solution::find_longest_word(
                "abpcplea".to_string(),
                vec!["a".to_string(), "b".to_string(), "c".to_string()]
            ),
            "a".to_string()
        );
    }
}
