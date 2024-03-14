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
                '+' | '-' => {
                    let info = TokenInfo {
                        length: 1,
                        line: self.line,
                        start: self.line_pos,
                    };

                    self.line_pos += 1;

                    match op {
                        '+' => tokens.push(Token {
                            r#type: TokenType::Plus,
                            value: None,
                            info,
                        }),
                        '-' => tokens.push(Token {
                            r#type: TokenType::Minus,
                            value: None,
                            info,
                        }),
                        _ => unreachable!(),
                    }
                }
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
            info: TokenInfo {
                length: 1,
                line: self.line,
                start: self.line_pos,
            },
        });

        tokens
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

        let info = TokenInfo {
            length: number.len(),
            line: self.line,
            start: self.line_pos,
        };

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
