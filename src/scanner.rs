use serde::{Deserialize, Serialize};

use crate::{
    token::{Token, TokenInfo, TokenType},
    value::Value,
};

pub struct Scanner {
    source: Vec<char>,
    index: usize,
    line: usize,
    line_pos: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Factor {
    Add,
    Sub,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            index: 0,
            line: 0,
            line_pos: 0,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.is_at_end() {
            let op = self.consume();
            match op {
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    let info = self.token_info(1);

                    tokens.push(Token {
                        value: None,
                        info,
                        r#type: match op {
                            '+' => TokenType::Plus,
                            '-' => TokenType::Minus,
                            '*' => TokenType::Star,
                            '/' => TokenType::Slash,
                            '(' => TokenType::LeftParen,
                            ')' => TokenType::RightParen,
                            _ => unreachable!(),
                        },
                    });
                }
                '>' | '<' | '=' | '!' => self.relational(&mut tokens),
                '0'..='9' => tokens.push(self.number()),
                '\n' => {
                    self.line += 1;
                    self.line_pos = 0;
                }
                _ => {}
            }
        }

        tokens.push(Token {
            r#type: TokenType::Eof,
            value: None,
            info: self.token_info(1),
        });

        tokens
    }

    fn relational(&mut self, tokens: &mut Vec<Token>) {
        let c = self.prev().unwrap();

        tokens.push(if self.r#match('=') {
            let rel_eq = format!("{}=", c);
            self.index += 1;
            Token {
                value: None,
                r#type: TokenType::from(rel_eq.as_str()),
                info: self.token_info(2),
            }
        } else {
            Token {
                value: None,
                r#type: TokenType::from(c),
                info: self.token_info(1),
            }
        });
    }

    fn r#match(&mut self, expected: char) -> bool {
        !(self.is_at_end() || self.peek() != Some(expected))
    }

    fn number(&mut self) -> Token {
        let mut is_float = false;
        let mut number = String::new();
        number.push(self.prev().unwrap());

        while let Some('.' | '0'..='9') = self.peek() {
            let res = self.consume();
            number.push(res);
            if res == '.' && is_float {
                panic!("Found two decimals in a float");
            }
            if res == '.' && !is_float {
                is_float = true;
            }
        }

        let info = self.token_info(number.len());

        self.line_pos += number.len();

        match is_float {
            true => Token {
                r#type: TokenType::Float,
                value: Some(Value::Float(number.parse().unwrap())),
                info,
            },
            false => Token {
                r#type: TokenType::Integer,
                value: Some(Value::Integer(number.parse().unwrap())),
                info,
            },
        }
    }

    fn consume(&mut self) -> char {
        let res = self.source[self.index];
        self.index += 1;
        res
    }

    fn prev(&self) -> Option<char> {
        if self.index > 0 {
            Some(self.source[self.index - 1])
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        if !self.is_at_end() {
            Some(self.source[self.index])
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.source.len()
    }

    fn peek_next(&self) -> Option<char> {
        if self.index + 1 < self.source.len() {
            Some(self.source[self.index + 1])
        } else {
            None
        }
    }

    fn token_info(&mut self, length: usize) -> TokenInfo {
        self.line_pos += length;
        TokenInfo {
            start: self.line_pos,
            length,
            line: self.line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_1() {
        let mut scanner = Scanner::new(" 20.0 + 30.0 - 3 ".to_string());
        assert_yaml_snapshot!(scanner.scan());
    }

    #[test]
    fn test_2() {
        let input = "93367-76920+596894-231722-8350-3517484393530.0-65+710".to_string();
        let mut scanner = Scanner::new(input);
        assert_yaml_snapshot!(scanner.scan());
    }
}
