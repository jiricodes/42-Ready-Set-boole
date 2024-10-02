//! You must write a function that takes an integer n and returns
//! its equivalent in Gray code.
//! - [Wiki](https://en.wikipedia.org/wiki/Gray_code)

pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::gray_code;

    #[test]
    fn test_gray_code() {
        let n = 0;
        assert_eq!(0, gray_code(n));

        let n = 1;
        assert_eq!(1, gray_code(n));

        let n = 2;
        assert_eq!(3, gray_code(n));

        let n = 3;
        assert_eq!(2, gray_code(n));

        let n = 4;
        assert_eq!(6, gray_code(n));

        let n = 5;
        assert_eq!(7, gray_code(n));

        let n = 6;
        assert_eq!(5, gray_code(n));

        let n = 7;
        assert_eq!(4, gray_code(n));

        let n = 8;
        assert_eq!(12, gray_code(n));
    }
}
