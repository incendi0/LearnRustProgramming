struct Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        if original.len() != m * n {
            Vec::new()
        } else {
            original.chunks(n).map(|chk| chk.to_vec()).collect()
        }
    }
}
