pub fn greedy_activity_select(xs: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    xs.sort_by(|a, b| a.1.cmp(&b.1));
    let mut ret = vec![xs[0]];
    let mut k = 0;
    for i in 2..xs.len() {
        if xs[i].0 >= xs[k].1 {
            ret.push(xs[i]);
            k = i;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greedy_activity_select_works() {
        let mut xs = vec![
            (1, 4),
            (3, 5),
            (0, 6),
            (5, 7),
            (3, 9),
            (5, 9),
            (6, 10),
            (8, 11),
            (8, 12),
            (2, 14),
            (12, 16),
        ];
        let activities = greedy_activity_select(&mut xs);
        assert_eq!(activities, vec![(1, 4), (5, 7), (8, 11), (12, 16)]);
    }
}
