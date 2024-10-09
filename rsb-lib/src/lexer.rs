use std::str::Chars;
use crate::Token;

pub trait LexerTrait<T> where T: Token {
    fn next_char(&mut self) -> Option<char>;

    fn next_token(&mut self) -> T {
        if let Some(value) = self.next_char() {
            value.into()
        } else {
            T::eof()
        }
    }
}

pub struct Lexer<'a> {
    iter: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars(),
        }
    }
}

impl<'a, T: Token> LexerTrait<T> for Lexer<'a> {
    fn next_char(&mut self) -> Option<char> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::BoolToken;

    #[test]
    fn test_next_booltoken() {
        let input = "01!&|^>=7";
        let mut lexer = Lexer::new(&input);

        let token: BoolToken = lexer.next_token();
        assert_eq!(token, BoolToken::False);
        let token: BoolToken = lexer.next_token();
        assert_eq!(token, BoolToken::True);
        let token: BoolToken = lexer.next_token();
        assert_eq!(token, BoolToken::Negation);
        let token: BoolToken = lexer.next_token();
        assert_eq!(token, BoolToken::And);
        let token: BoolToken = lexer.next_token();
        assert_eq!(token, BoolToken::Or);
        let token: BoolToken = lexer.next_token();
        assert_eq!(token, BoolToken::Xor);
        let token: BoolToken = lexer.next_token();
        assert_eq!(token, BoolToken::Cond);
        let token: BoolToken = lexer.next_token();
        assert_eq!(token, BoolToken::Eq);
        let token: BoolToken = lexer.next_token();
        assert_eq!(token, BoolToken::Illegal);
        let token: BoolToken = lexer.next_token();
        assert_eq!(token, BoolToken::EOF);
    }
}
