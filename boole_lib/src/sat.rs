use crate::boolrpntree::BoolRpnTree;

pub fn sat(formula: &str) -> bool {
    // Parse the formula and retrieve variables
    let mut labels: Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();
    labels.sort();
    labels.dedup();
    let l = labels.len();
    // loop over all possible combinations
    for i in 0..2u32.pow(l as u32) {
        let mut current = String::from(formula);
        for (n, c) in labels.iter().enumerate() {
            let val = if i & (1 << (l - 1 - n)) != 0 {
                "1"
            } else {
                "0"
            };
            current = current.replace(*c, val);
        }
        if BoolRpnTree::from(current.as_str()).result() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject() {
        let tests: Vec<(bool, &str)> = vec![
            (true, "AB|"),
            (true, "AB&"),
            (false, "AA!&"),
            (false, "AA^"),
        ];
        for (exp, test) in tests.iter() {
            assert_eq!(*exp, sat(*test), "Case \"{}\"", *test);
        }
    }
}
