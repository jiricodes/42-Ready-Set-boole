use rsb_lib::negation_normal_form;

fn main() {
    let formula = "A!!";
    println!("Original: {}", formula);
    println!("Expected: A");
    println!("NNF: {}\n", negation_normal_form(formula));
    let formula = "AB&!";
    println!("Original: {}", formula);
    println!("Expected: A!B!|");
    println!("NNF: {}\n", negation_normal_form(formula));
    let formula = "AB|!";
    println!("Original: {}", formula);
    println!("Expected: A!B!&");
    println!("NNF: {}\n", negation_normal_form(formula));
    let formula = "AB>";
    println!("Original: {}", formula);
    println!("Expected: A!B|");
    println!("NNF: {}\n", negation_normal_form(formula));
    let formula = "AB=";
    println!("Original: {}", formula);
    println!("Expected: AB&A!B!&|");
    println!("NNF: {}\n", negation_normal_form(formula));
    let formula = "AB|C&!";
    println!("Original: {}", formula);
    println!("Expected: A!B!&C!|");
    println!("NNF: {}\n", negation_normal_form(formula));
    let formula = "AB>!";
    println!("Original: {}", formula);
    println!("Expected: AB!&");
    println!("NNF: {}\n", negation_normal_form(formula));
}
