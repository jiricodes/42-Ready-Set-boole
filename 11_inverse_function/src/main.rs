use rsb_lib::reverse_map;

fn main() {
    let r = 0.011;
    println!("Original {}", r);
    let (x, y) = reverse_map(r);
    println!("({}, {})", x, y);
}
