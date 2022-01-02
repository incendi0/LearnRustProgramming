pub fn check_string(s: String) -> bool {
    let n = s.len();
    let (mut ai, mut bi) = (n, n);
    for (i, ch) in s.chars().enumerate() {
        if ch == 'a' {
            ai = i;
        } else {
            if bi == n {
                bi = i;
            }
        }
    }
    if ai == n || bi == n {
        true
    } else {
        ai < bi
    }
}

pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut total = 0;
    let mut last = 0;
    for s in bank {
        let curr = s.chars().filter(|&ch| ch == '1').count() as i32;
        if curr != 0 {
            total += last * curr;
            last = curr;
        }
    }
    total
}

pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
    let mut asteroids = asteroids;
    asteroids.sort();
    let mut mass = mass as i64;
    for e in asteroids {
        if mass < e as i64 {
            return false;
        } else {
            mass += e as i64;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_string_works() {
        assert!(check_string("aaabbb".to_string()));
        assert!(!check_string("abab".to_string()));
        assert!(check_string("bbb".to_string()));
        assert!(check_string("aaa".to_string()));
    }

    #[test]
    fn number_of_beams_works() {
        assert_eq!(
            number_of_beams(vec![
                "011001".into(),
                "000000".into(),
                "010100".into(),
                "001000".into()
            ]),
            8
        );
        assert_eq!(
            number_of_beams(vec!["000".into(), "111".into(), "000".into()]),
            0
        );
    }

    #[test]
    fn asteroids_destroyed_works() {
        assert!(asteroids_destroyed(10, vec![3, 9, 19, 5, 21]));
        assert!(!asteroids_destroyed(5, vec![4, 9, 23, 4]));
    }
}
