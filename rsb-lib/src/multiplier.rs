//! 01 Multiplier
//! The goal is the same as the previous exercise (00 Adder), except
//! the returned natural number equals a * b. The only operations you’re
//! allowed to use are:
//!     - & (bitwise AND)
//!     - | (bitwise OR)
//!     - ^ (bitwise XOR)
//!     - << (left shift)
//!     - >> (right shift)
//!     - = (assignment)
//!     - ==, !=, <, >, <=, >= (comparison operators)
//! The incrementation operator (++ or += 1) is allowed only to increment
//! the index of a loop and must not be used to compute the result itself.

use crate::adder;

/// For the solution I decided to go with the Russian peasant method
/// - [Wiki](https://en.wikipedia.org/wiki/Ancient_Egyptian_multiplication#Russian_peasant_multiplication)
/// - [Geeks for Geeks](https://www.geeksforgeeks.org/russian-peasant-multiply-two-numbers-using-bitwise-operators/)
/// - [Nice explanation](https://www.themathdoctors.org/russian-peasant-multiplication-how-and-why/)
/// This solution doesn't prevent overflow.
pub fn multiplier(a: u32, b:u32) -> u32 {
    let mut res: u32 = 0;
    let mut a = a;
    let mut b = b;
    while b > 0 {
        if b & 1 != 0 {
            res = adder(res, a)
        }
        a = a << 1;
        b = b >> 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplier() {
        let mut a: u32;
        let mut b: u32;

        a = 0;
        b = 0;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);

        a = 0;
        b = 1;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);

        a = 1;
        b = 0;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);

        a = 1;
        b = 1;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);

        a = 7;
        b = 3;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);

        a = 3;
        b = 7;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);

        a = 123;
        b = 123;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);

        a = 0b101010;
        b = 0b010101;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);

        a = 0b111111;
        b = 0b010101;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);

        a = 0b111111;
        b = 0b011101;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);

        a = 0b111111;
        b = 0b011001;
        assert_eq!(a * b, multiplier(a, b), "{} * {}", a, b);
    }

    #[test]
    #[should_panic]
    fn test_multiplier_panic() {
        let a = u32::MAX / 4;
        let b = 5;
        multiplier(a, b);
    }
}
