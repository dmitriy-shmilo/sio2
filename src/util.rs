// Fits `i` into `lower..upper` range, wrapping its value around
// if necessary.
// See https://stackoverflow.com/a/707426/575979
#[inline]
pub fn wrap(mut i: i32, lower: i32, upper: i32) -> i32 {
    let range_size = upper - lower;

    if i < lower {
        i += range_size * ((lower - i) / range_size + 1);
    }

    lower + (i - lower) % range_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_wrap() {
        assert_eq!(wrap(-1, 0, 100), 99);
    }

    #[test]
    fn test_positive_wrap() {
        assert_eq!(wrap(100, 0, 100), 0);
    }

    #[test]
    fn test_no_wrap() {
        assert_eq!(wrap(10, 0, 100), 10);
    }
}