pub fn adder(a: u32, b: u32) -> u32 {
    let mut carry = 0;
    let mut ret = 0;
    for i in 0..32 {
        // align the bits of interest
        let a0 = (a >> i) & 1;
        let b0 = (b >> i) & 1;
        // get the added bit value and put it to correct place in result
        ret = ret | ((a0 ^ b0 ^ carry) << i);
        // check carry, carry is 1 if
        //  a0 & b0 - we created carry in this location and previous carry either didn't exist or was consumed
        //  carry==1 && a0 ^ b0 - we had carry, but not consumed
        carry = (a0 & b0) | ((a0 ^ b0) & carry);
    }
    if carry == 1 {
        panic!("attempt to add with overflow");
    }
    ret
}

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
