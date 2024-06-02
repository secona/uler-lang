use crate::ast::expressions::Expression;
use crate::ast::Identifier;
use crate::token;

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Expression,
}

impl std::fmt::Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "LetStatement(name={}, value={})",
            self.name.to_string(),
            self.value.to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, lexer, parser, testing, token};

    use super::LetStatement;

    #[test]
    fn to_string() {
        testing::stringify!(
            LetStatement {
                token: token::Token::Let,
                name: ast::Identifier {
                    token: token::Token::Ident(String::from("x")),
                    value: String::from("x"),
                },
                value: ast::Expression::IntegerLiteral(ast::expressions::IntegerLiteral {
                    token: token::Token::Int(String::from("5")),
                    value: 5,
                }),
            },
            String::from("let x = 5;")
        );

        testing::stringify!(
            LetStatement {
                token: token::Token::Let,
                name: ast::Identifier {
                    token: token::Token::Ident(String::from("myVar")),
                    value: String::from("myVar"),
                },
                value: ast::Expression::Identifier(ast::expressions::Identifier {
                    token: token::Token::Ident(String::from("anotherVar")),
                    value: String::from("anotherVar"),
                }),
            },
            String::from("let myVar = anotherVar;")
        );
    }

    #[test]
    fn parsing() {
        let input = "let x = 5;".to_owned().into_bytes().into_boxed_slice();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program().expect("got parser errors");

        println!(
            "{}",
            program
                .statements
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        assert_eq!(program.statements.len(), 1);

        let stmt = testing::as_variant!(&program.statements[0], ast::Statement::LetStatement);

        assert_eq!(stmt.name.token, token::Token::Ident(String::from("x")));
        assert_eq!(stmt.name.value, String::from("x"));

        testing::expr_variant!(&stmt.value, ast::Expression::IntegerLiteral = 5);
    }
}
