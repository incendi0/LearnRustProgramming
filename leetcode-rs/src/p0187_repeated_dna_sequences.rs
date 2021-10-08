use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut count = HashMap::new();
        let xs: Vec<char> = s.chars().collect();
        for sub in xs.windows(10) {
            *count.entry(sub).or_insert(0) += 1;
        }
        count
            .into_iter()
            .filter(|&(_, v)| v > 1)
            .map(|(s, _)| s.iter().collect::<String>())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn find_repeated_dna_sequences_works() {
        let mut xs =
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string());
        xs.sort();
        assert_eq!(xs, vec!["AAAAACCCCC", "CCCCCAAAAA"]);
        assert_eq!(
            Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()),
            vec!["AAAAAAAAAA"]
        );
    }
}
