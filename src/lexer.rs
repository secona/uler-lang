use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: Option<&'a u8>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        let tok: Token;

        match self.ch {
            None => tok = Token::EOF,
            Some(ch) => match ch {
                b'=' => match self.peek_char() {
                    Some(b'=') => {
                        tok = Token::Eq;
                        self.read_char();
                    }
                    _ => tok = Token::Assign,
                },
                b'!' => match self.peek_char() {
                    Some(b'=') => {
                        tok = Token::NotEq;
                        self.read_char();
                    }
                    _ => tok = Token::Bang,
                },
                b':' => match self.peek_char() {
                    Some(b'=') => {
                        tok = Token::Walrus;
                        self.read_char();
                    }
                    _ => tok = Token::Illegal(" ".into()),
                },
                b';' => tok = Token::Semicolon,
                b'(' => tok = Token::LParen,
                b')' => tok = Token::RParen,
                b',' => tok = Token::Comma,
                b'+' => tok = Token::Plus,
                b'-' => tok = Token::Minus,
                b'*' => tok = Token::Asterisk,
                b'/' => tok = Token::Slash,
                b'%' => tok = Token::Percent,
                b'>' => tok = Token::GT,
                b'<' => tok = Token::LT,
                b'{' => tok = Token::LBrace,
                b'}' => tok = Token::RBrace,
                b'"' => {
                    let literal = self.read_string();
                    tok = Token::String(String::from_utf8(literal.to_vec()).unwrap());
                }
                _ => {
                    if self.is_letter() {
                        tok = self.read_identifier();
                        return tok;
                    } else if self.is_digit() {
                        tok = self.read_number();
                        return tok;
                    } else {
                        tok = Token::Illegal(" ".into())
                    }
                }
            },
        };

        self.read_char();
        tok
    }

    pub fn read_char(&mut self) -> Option<&'a u8> {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(&self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
        self.ch
    }

    pub fn peek_char(&self) -> Option<&'a u8> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(&self.input[self.read_position])
        }
    }

    pub fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.ch {
                Some(b'#') => self.skip_comment(),
                Some(b' ' | b'\t' | b'\n' | b'\r') => {
                    self.read_char();
                }
                _ => break,
            };
        }
    }

    pub fn skip_comment(&mut self) {
        while let Some(b'#') = self.ch {
            loop {
                match self.ch {
                    Some(b'\n') | None => {
                        self.read_char();
                        break;
                    }
                    _ => self.read_char(),
                };
            }
        }
    }

    pub fn read_string(&mut self) -> &'a [u8] {
        let position = self.position + 1;

        loop {
            self.read_char();
            match self.ch {
                Some(b'"') | Some(0) => break,
                _ => (),
            }
        }

        &self.input[position..self.position]
    }

    pub fn read_identifier(&mut self) -> Token {
        let position = self.position;

        while self.is_letter() {
            self.read_char();
        }

        Token::from(&self.input[position..self.position])
    }

    pub fn is_letter(&self) -> bool {
        if let Some(ch) = self.ch {
            *ch >= b'a' && *ch <= b'z' || *ch >= b'A' && *ch <= b'Z' || *ch == b'_'
        } else {
            false
        }
    }

    pub fn read_number(&mut self) -> Token {
        let position = self.position;

        while self.is_digit() {
            self.read_char();
        }

        let num = &self.input[position..self.position];
        let num = std::str::from_utf8(num).unwrap();
        Token::Int(String::from(num))
    }

    pub fn is_digit(&self) -> bool {
        match self.ch {
            Some(ch) => *ch >= b'0' && *ch <= b'9',
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::Token;

    #[test]
    fn tokens() {
        let input = b"=+(){},;!-/*5;5 < 10 > 5;:=";

        let expected: [Token; 21] = [
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::LT,
            Token::Int(String::from("10")),
            Token::GT,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Walrus,
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert_eq!(tok, exp);
        }
    }

    #[test]
    fn multichar_token() {
        let input = b"five := 5;
ten := 10;

add := fn(x, y) {
    x + y;
};

result := add(five, ten);

\"Hello, World!\"";

        let expected: [Token; 34] = [
            Token::Ident(String::from("five")),
            Token::Walrus,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Ident(String::from("ten")),
            Token::Walrus,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Ident(String::from("add")),
            Token::Walrus,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Ident(String::from("result")),
            Token::Walrus,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::String(String::from("Hello, World!")),
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert_eq!(tok, exp);
        }
    }

    #[test]
    fn if_else() {
        let input = b"if (5 < 10) {
    return true;
} else {
    return false;
}";

        let expected: [Token; 18] = [
            Token::If,
            Token::LParen,
            Token::Int(String::from("5")),
            Token::LT,
            Token::Int(String::from("10")),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert_eq!(tok, exp);
        }
    }
    #[test]
    fn equality() {
        let input = b"10 == 10;\n9 != 10;";

        let expected: [Token; 8] = [
            Token::Int(String::from("10")),
            Token::Eq,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Int(String::from("9")),
            Token::NotEq,
            Token::Int(String::from("10")),
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input);

        for exp in expected {
            let tok = lexer.next_token();
            println!("tok={:?} exp={:?}", tok, exp);
            assert_eq!(tok, exp);
        }
    }
}
