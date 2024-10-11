use rsb_lib::eval_formula;

fn main() {
    let formula = "1011||=";
    println!("{}\n{}", formula, eval_formula(formula));
}
