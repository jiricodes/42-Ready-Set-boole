
//! Adder
//! You must write a function that takes as parameters two natural numbers
//! a and b and returns one natural number that equals a + b.
//! However the only operations you’re allowed to use are:
//!     - & (bitwise AND)
//!     - | (bitwise OR)
//!     - ^ (bitwise XOR)
//!     - • << (left shift)
//!     - • >> (right shift)
//!     - • = (assignment)
//!     - • ==, !=, <, >, <=, >= (comparison operators)
//!
//! The incrementation operator (++ or += 1) is allowed only to increment
//! the index of a loop and must not be used to compute the result itself.

/// This is the simplest solution to an adder function.
/// Since we are working with 32 bits, there's no stack overflow danger.
/// O(32) -> O(1)
/// This doesn't protect from an integer overflow.
pub fn adder(a: u32, b: u32) -> u32 {
    let carry: u32 = (a & b) << 1;
    let result: u32 = a ^ b;
    if carry == 0 {
        return result;
    }
    adder (carry, result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adder() {
        let mut a;
        let mut b;

        a = 0;
        b = 0;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 1;
        b = 0;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 0;
        b = 1;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 1;
        b = 1;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 7;
        b = 3;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 3;
        b = 7;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 123;
        b = 123;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 0b101010;
        b = 0b010101;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 0b111111;
        b = 0b010101;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 0b111111;
        b = 0b011101;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 0b111111;
        b = 0b011001;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 2147483647;
        b = 1073741823;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);

        a = 1073741823;
        b = 2147483647;
        assert_eq!(a + b, adder(a, b), "{} + {}", a, b);
    }

    #[test]
    #[should_panic]
    fn test_adder_panic() {
        let a = u32::MAX;
        let b = 5;
        adder(a, b);
    }
}
