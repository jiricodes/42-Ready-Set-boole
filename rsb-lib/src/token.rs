pub trait Token: From<char> {
    fn eof() -> Self;
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoolToken {
    False,    // 0 ⊥ false
    True,     // 1 ⊤ true
    Negation, // ! ¬ Negation
    And,      // & ∧ Conjunction
    Or,       // | ∨ Disjunction
    Xor,      // ˆ ⊕ Exclusive disjunction
    Cond,     // > ⇒ Material condition
    Eq,       // = ⇔ Logical equivalence
    EOF,
    Illegal,
}

impl Token for BoolToken {
    fn eof() -> Self {
        Self::EOF
    }
}

impl From<char> for BoolToken {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::False,
            '1' => Self::True,
            '!' => Self::Negation,
            '&' => Self::And,
            '|' => Self::Or,
            '^' => Self::Xor,
            '>' => Self::Cond,
            '=' => Self::Eq,
            _ => Self::Illegal,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CharToken {
    Value(char),    // A..Z
    Negation, // ! ¬ Negation
    And,      // & ∧ Conjunction
    Or,       // | ∨ Disjunction
    Xor,      // ˆ ⊕ Exclusive disjunction
    Cond,     // > ⇒ Material condition
    Eq,       // = ⇔ Logical equivalence
    EOF,
    Illegal,
}

impl Token for CharToken {
    fn eof() -> Self {
        Self::EOF
    }
}

impl From<char> for CharToken {
    fn from(value: char) -> Self {
        match value {
            'A'..'Z' => Self::Value(value),
            '!' => Self::Negation,
            '&' => Self::And,
            '|' => Self::Or,
            '^' => Self::Xor,
            '>' => Self::Cond,
            '=' => Self::Eq,
            _ => Self::Illegal,
        }
    }
}
