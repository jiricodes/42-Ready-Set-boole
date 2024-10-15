use rsb_lib::conjunctive_normal_form;

fn main() {
    let formula = "AB&!";
    println!("Original: {}", formula);
    println!("Expected: A!B!|");
    println!("CNF: {}\n", conjunctive_normal_form(formula));
    let formula = "AB|!";
    println!("Original: {}", formula);
    println!("Expected: A!B!&");
    println!("CNF: {}\n", conjunctive_normal_form(formula));
    let formula = "AB|C&";
    println!("Original: {}", formula);
    println!("Expected: AB|C&");
    println!("CNF: {}\n", conjunctive_normal_form(formula));
    let formula = "AB|C|D|";
    println!("Original: {}", formula);
    println!("Expected: ABCD|||");
    println!("CNF: {}\n", conjunctive_normal_form(formula));
    let formula = "AB&C&D&";
    println!("Original: {}", formula);
    println!("Expected: ABCD&&&");
    println!("CNF: {}\n", conjunctive_normal_form(formula));
    let formula = "AB&!C!|";
    println!("Original: {}", formula);
    println!("Expected: A!B!C!||");
    println!("CNF: {}\n", conjunctive_normal_form(formula));
    let formula = "AB|!C!&";
    println!("Original: {}", formula);
    println!("Expected: A!B!C!&&");
    println!("CNF: {}\n", conjunctive_normal_form(formula));
    let formula = "ABCD|||";
    println!("Original: {}", formula);
    println!("Expected: ABCD|||");
    println!("CNF: {}\n", conjunctive_normal_form(formula));
}
