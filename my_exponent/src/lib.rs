// my_exponent/src/lib.rs

pub fn power(base: i64, exponent: usize) -> i64 {
    let mut res = 1;
    if exponent == 0 {
        return 1;
    }
    for _ in 0..exponent {
        res *= base as i64;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::power;
    #[test]
    fn minus_two_raised_three_is_minus_eight() {
        assert_eq!(power(-2, 3), -8);
    }
}
