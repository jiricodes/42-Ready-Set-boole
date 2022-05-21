use crate::boolrpntree::BoolRpnTree;

fn print_header(labels: &Vec<char>) {
    let mut line = String::new();
    let mut underscore = String::new();
    for c in labels.iter() {
        line += &format!("| {} ", c);
        underscore += "|---";
    }
    println!("{}| = |", line);
    println!("{}|---|", underscore);
}

pub fn print_truth_table(formula: &str) {
    let mut labels: Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();
    labels.sort();
    labels.dedup();
    let l = labels.len();
    print_header(&labels);
    for i in 0..2u32.pow(l as u32) {
        let mut current = String::from(formula);
        let mut table_line = String::new();
        for (n, c) in labels.iter().enumerate() {
            let val = if i & (1 << (l - 1 - n)) != 0 {
                table_line += "| 1 ";
                "1"
            } else {
                table_line += "| 0 ";
                "0"
            };
            current = current.replace(*c, val);
        }
        let ret = if BoolRpnTree::from(current.as_str()).result() {
            "1"
        } else {
            "0"
        };
        println!("{}| {} |", table_line, ret);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        print_truth_table("AB&C|D^B!=");
    }

    #[test]
    fn subject() {
        print_truth_table("AB&C|");
    }
}
