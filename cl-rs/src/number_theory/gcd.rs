pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: i32, b: i32) -> i32 {
    let g = gcd(a, b);
    a / g * b
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(14, 21), 7);
        assert_eq!(gcd(21, 14), 7);
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(2, 4), 4);
        assert_eq!(lcm(14, 21), 42);
        assert_eq!(lcm(21, 14), 42);
    }
}
