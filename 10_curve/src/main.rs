//! Let f be a function and let A be a set such as:
//!     f : (x, y) ∈ [[0; 2^16 − 1]]^2 ⊂ N^2 → A
//!     A ⊂ [0; 1] ⊂ R
//! Where:
//!     card(A) = card([[0; 216 − 1]]^2)
//!
//! https://doi.org/10.1063/1.1751381

/// https://graphics.stanford.edu/~seander/bithacks.html#InterleaveBMN
fn interleave(x: u16, y: u16) -> u32 {
    const B: [u32; 4] = [0x55555555, 0x33333333, 0x0F0F0F0F, 0x00FF00FF];
    const S: [u32; 4] = [1, 2, 4, 8];

    let mut x: u32 = x as u32;
    let mut y: u32 = y as u32;

    x = (x | (x << S[3])) & B[3];
    x = (x | (x << S[2])) & B[2];
    x = (x | (x << S[1])) & B[1];
    x = (x | (x << S[0])) & B[0];

    y = (y | (y << S[3])) & B[3];
    y = (y | (y << S[2])) & B[2];
    y = (y | (y << S[1])) & B[1];
    y = (y | (y << S[0])) & B[0];

    x | (y << 1)
}

fn de_interleave(x: u32) -> u16 {
       let mut x = x & 0x55555555;
    x = (x | (x >> 1)) & 0x33333333;
    x = (x | (x >> 2)) & 0x0F0F0F0F;
    x = (x | (x >> 4)) & 0x00FF00FF;
    x = (x | (x >> 8)) & 0x0000FFFF;
    x as u16
}

fn map(x: u16, y: u16) -> f64 {
    interleave(x, y) as f64 / u32::MAX as f64
}

fn reverse_map(n: f64) -> (u16, u16) {
    let n = (n * u32::MAX as f64) as u32;
    (de_interleave(n), de_interleave(n>>1))

}

fn main() {
    let x = 0b0101;
    let y = 0b1010;
    println!("Original ({}; {})", x, y);
    let res = map(x, y);
    println!("{}", res);
    let (xr, yr) = reverse_map(res);
    println!("Reversed ({}; {})",xr, yr);
}


#[cfg(test)]
mod test {
    use super::{interleave, map, reverse_map};
    use rayon::prelude::*;

    #[test]
    fn test_interleave() {
        let x: u16 = 0b1111;
        let y: u16 = 0b0000;
        let res: u32 = 0b01010101;
        assert_eq!(res, interleave(x, y));


        let x: u16 = 0b1011;
        let y: u16 = 0b0010;
        let res: u32 = 0b01001101;
        assert_eq!(res, interleave(x, y));

        let x: u16 = 0b1011101110111011;
        let y: u16 = 0b0010110101000101;
        let res: u32 = 0b01001101111001110110010101100111;
        assert_eq!(res, interleave(x, y));
    }

    #[test]
    fn mega() {
        let _ = (0..u16::MAX).into_par_iter().map( |x| {
            for y in 0..u16::MAX {
                let res = map(x, y);
                let reversed = reverse_map(res);
                assert_eq!((x, y), reversed);
            }
        });
    }
}
