struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ret = 0;
        for idx in 1..n - 1 {
            if nums[idx] == nums[idx - 1] {
                continue;
            }
            let mut prev = idx - 1;
            while prev > 0 && nums[prev] == nums[idx] {
                prev -= 1;
            }
            let mut next = idx + 1;
            while next < n && nums[next] == nums[idx] {
                next += 1;
            }
            if next >= n || prev == 0 && nums[prev] == nums[idx] {
                continue;
            }
            if nums[prev] < nums[idx] && nums[next] < nums[idx] {
                ret += 1;
            }
            if nums[prev] > nums[idx] && nums[next] > nums[idx] {
                ret += 1;
            }
        }
        ret
    }

    pub fn count_collisions(directions: String) -> i32 {
        let mut xs = directions.as_bytes().to_vec();
        let mut idx = 0;
        let n = xs.len();
        let mut count = 0;
        while idx < n {
            if idx + 1 < n && xs[idx] == b'R' && xs[idx + 1] == b'L' {
                count += 2;
                xs[idx] = b'S';
                xs[idx + 1] = b'S';
                idx += 2;
            } else {
                idx += 1;
            }
        }
        idx = 0;
        while idx < n {
            if xs[idx] == b'S' {
                // count prev R
                if idx > 0 {
                    let mut prev = idx - 1;
                    while prev >= 1 && xs[prev] == b'R' {
                        count += 1;
                        prev -= 1;
                    }
                    if prev == 0 && xs[prev] == b'R' {
                        count += 1;
                    }
                }
                // count next L
                let mut next = idx + 1;
                while next < n && xs[next] == b'L' {
                    count += 1;
                    next += 1;
                }
                idx = next;
            } else {
                idx += 1;
            }
        }
        count
    }

    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        const N: i32 = 12;
        let bob_arrows: Vec<i32> = alice_arrows.iter().map(|&d| d + 1).collect();
        let mut maximum = 0;
        let mut ret = vec![0; N as usize];
        for comb in 1..(1 << N) {
            let mut sum_arrow = 0;
            let mut arrows = vec![0; N as usize];
            let mut sum_point = 0;
            for i in 0..12 {
                if comb & (1 << i) > 0 {
                    sum_arrow += bob_arrows[i];
                    arrows[i] = bob_arrows[i];
                    sum_point += i as i32;
                }
            }
            if sum_arrow <= num_arrows && sum_point > maximum {
                maximum = sum_point;
                ret = arrows;
            }
        }
        ret[0] += num_arrows - ret.iter().sum::<i32>();
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::contests::w285::Solution;

    #[test]
    fn count_hill_valley_works() {
        assert_eq!(Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]), 3);
        assert_eq!(Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]), 0);
    }

    #[test]
    fn count_collisions_works() {
        assert_eq!(Solution::count_collisions("RLRSLL".into()), 5);
        assert_eq!(Solution::count_collisions("LLRR".into()), 0);
    }

    #[test]
    fn maximum_bob_points_works() {
        assert_eq!(
            Solution::maximum_bob_points(9, vec![1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0]),
            [0, 0, 0, 0, 1, 1, 0, 0, 1, 2, 3, 1]
        );
        assert_eq!(
            Solution::maximum_bob_points(3, vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2]),
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0]
        );
        // assert_eq!(Solution::maximum_bob_points(89, vec![3,2,28,1,7,1,16,7,3,13,3,5]), [0,0,0,0,0,0,0,0,1,1,1,0]);
    }
}
