struct Solution {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut xs: Vec<(i32, i32)> = mat
            .iter()
            .enumerate()
            .map(|(i, row)| (i as i32, row.iter().sum()))
            .collect();
        // sort_by is stable
        xs.sort_by(|l, r| l.1.cmp(&r.1));
        xs.into_iter().take(k as usize).map(|(i, _)| i).collect()
    }
}
