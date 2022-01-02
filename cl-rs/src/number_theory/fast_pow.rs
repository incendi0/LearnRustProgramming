pub fn fast_pow(base: i32, exp: i32, m: i32) -> i32 {
    if exp == 0 {
        1
    } else if exp % 2 == 1 {
        fast_pow(base, exp - 1, m) * base % m
    } else {
        let h = fast_pow(base, exp / 2, m);
        h * h % m
    }
}

pub fn fast_pow_iter(base: i32, exp: i32, m: i32) -> i32 {
    let mut pow = 1;
    let mut exp = exp;
    let mut base = base;
    while exp > 0 {
        if (exp & 1) == 1 {
            pow = pow * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    pow
}
