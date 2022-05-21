use crate::adder;
/// Russian Peasant method
pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    let mut ret = 0;
    for _ in 0..32 {
        if b & 1 == 1 {
            ret = adder(ret, a);
        }
        b = b >> 1;
        a = a << 1;
    }
    ret
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
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);

        a = 0;
        b = 1;
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);

        a = 1;
        b = 0;
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);

        a = 1;
        b = 1;
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);

        a = 7;
        b = 3;
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);

        a = 3;
        b = 7;
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);

        a = 123;
        b = 123;
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);

        a = 0b101010;
        b = 0b010101;
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);

        a = 0b111111;
        b = 0b010101;
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);

        a = 0b111111;
        b = 0b011101;
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);

        a = 0b111111;
        b = 0b011001;
        assert_eq!(a * b, multiplier(a, b), "{} + {}", a, b);
    }

    #[test]
    #[should_panic]
    fn test_multiplier_panic() {
        let a = u32::MAX / 4;
        let b = 5;
        multiplier(a, b);
    }
}
