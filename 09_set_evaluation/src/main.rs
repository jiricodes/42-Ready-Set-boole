use rsb_lib::eval_set;

fn main() {
    let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![0, 3, 4]];
    let result = eval_set("AB&", sets);
    println!("{:?}", result);
}
