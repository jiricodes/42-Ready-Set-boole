use rsb_lib::map;

fn main() {
    let x = 0b0101;
    let y = 0b1010;
    println!("Original ({}; {})", x, y);
    let res = map(x, y);
    println!("{}", res);
}
