struct Solution;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        use std::cmp::max;
        let (mut t, mut ret) = (0, 'a');
        for (i, ch) in keys_pressed.chars().enumerate() {
            if i == 0 {
                ret = ch;
                t = release_times[i];
            } else {
                if t < release_times[i] - release_times[i - 1] {
                    ret = ch;
                    t = release_times[i] - release_times[i - 1];
                } else if t == release_times[i] - release_times[i - 1] {
                    ret = max(ret, ch);
                }
            }
        }
        ret
    }
}
