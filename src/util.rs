// Fits `i` into `lower..upper` range, wrapping its value around
// if necessary.
// See https://stackoverflow.com/a/707426/575979
use crate::{
    FIELD_WIDTH_F32,
    FIELD_HEIGHT_F32
};

#[inline]
pub fn wrap(mut i: i32, lower: i32, upper: i32) -> i32 {
    let range_size = upper - lower;

    if i < lower {
        i += range_size * ((lower - i) / range_size + 1);
    }

    lower + (i - lower) % range_size
}

#[inline]
pub fn window_size_to_scale(width: usize, height: usize) -> f32 {

    if width == 0 || height == 0 {
        1.
    } else if width < height {
        width as f32 / FIELD_WIDTH_F32
    } else {
        height as f32 / FIELD_HEIGHT_F32
    }

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