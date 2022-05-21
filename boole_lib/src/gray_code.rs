/// To convert binary to gray code, bring down the most siginificant digit
/// of the given binary number, because, the first digit or most siginificant
/// digit of the gray code number is same as the binary number
pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_code() {
        let mut n: u32;
        let mut expected: u32;

        n = 0;
        expected = 0;
        assert_eq!(expected, gray_code(n));

        n = 1;
        expected = 1;
        assert_eq!(expected, gray_code(n));

        n = 2;
        expected = 3;
        assert_eq!(expected, gray_code(n));

        n = 3;
        expected = 2;
        assert_eq!(expected, gray_code(n));

        n = 4;
        expected = 6;
        assert_eq!(expected, gray_code(n));

        n = 0b010111001;
        expected = 0b011100101;
        assert_eq!(expected, gray_code(n));

        n = 0b101011110101101110;
        expected = 0b111110001111011001;
        assert_eq!(expected, gray_code(n));
    }
}
