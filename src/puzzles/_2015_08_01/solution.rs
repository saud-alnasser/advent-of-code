use std::{iter::Peekable, str::Chars};

crate::puzzle!("2015_08_01");

#[derive(Debug)]
pub enum Token {
    Quote,
    Character(char),
    Escape(char),
    Hex(char),
}

struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn next(&mut self) -> Option<Token> {
        if let Some(c) = self.chars.next() {
            if c == ' ' {
                return self.next();
            }

            let peeked = self.chars.peek();

            if c == '\\' && peeked.is_some() && peeked.unwrap() == &'x' {
                self.chars.next();

                let mut hex = String::new();

                hex.push(self.chars.next().unwrap());
                hex.push(self.chars.next().unwrap());

                Some(Token::Hex(u8::from_str_radix(&hex, 16).unwrap() as char))
            } else if c == '\\'
                && peeked.is_some()
                && (peeked.unwrap() == &'\\' || peeked.unwrap() == &'"')
            {
                Some(Token::Escape(self.chars.next().unwrap()))
            } else if c == '"' {
                Some(Token::Quote)
            } else {
                Some(Token::Character(c))
            }
        } else {
            None
        }
    }
}

impl Solution for Puzzle {
    type Input = Vec<Token>;

    fn parse(input: &str) -> Self::Input {
        let mut tokenizer = Tokenizer::new(input);

        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        }

        tokens
    }

    fn solve(tokens: Self::Input) -> Option<String> {
        let mut represented = 0;
        let mut code = 0;

        for token in &tokens {
            match token {
                Token::Character(_) => {
                    represented += 1;
                    code += 1;
                }
                Token::Quote => {
                    code += 1;
                }
                Token::Escape(_) => {
                    represented += 1;
                    code += 2;
                }
                Token::Hex(_) => {
                    represented += 1;
                    code += 4;
                }
            }
        }

        Some(format!("{}", code - represented))
    }
}
