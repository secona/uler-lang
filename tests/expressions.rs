#[macro_use]
mod common;

use belalang::{ast, token};
use common::test_parse;

#[test]
fn boolean_true() {
    let program = test_parse("true;");

    assert_eq!(program.statements.len(), 1);

    let expr = as_variant!(&program.statements[0], ast::Statement::Expression);

    let bool_expr = as_variant!(&expr.expression, ast::Expression::Boolean);

    assert_eq!(bool_expr.value, true);
    assert_eq!(bool_expr.token, token::Token::True);
}

#[test]
fn boolean_false() {
    let program = test_parse("false;");

    assert_eq!(program.statements.len(), 1);

    let expr = as_variant!(&program.statements[0], ast::Statement::Expression);

    let bool_expr = as_variant!(&expr.expression, ast::Expression::Boolean);

    assert_eq!(bool_expr.value, false);
    assert_eq!(bool_expr.token, token::Token::False);
}

#[test]
fn call() {
    let program = test_parse("add(1, 2 * 3, 4 + 5);");

    assert_eq!(program.statements.len(), 1);

    let stmt = as_variant!(&program.statements[0], ast::Statement::Expression);

    let expr = as_variant!(&stmt.expression, ast::Expression::Call);

    expr_variant!(&*expr.function, ast::Expression::Identifier = "add");

    assert_eq!(expr.args.len(), 3);
    expr_variant!(&expr.args[0], ast::Expression::Integer = 1);
    expr_variant!(
        &expr.args[1], Infix => (
            ast::Expression::Integer = 2,
            token::Token::Asterisk,
            ast::Expression::Integer = 3
        )
    );
    expr_variant!(
        &expr.args[2], Infix => (
            ast::Expression::Integer = 4,
            token::Token::Plus,
            ast::Expression::Integer = 5
        )
    );
}

#[test]
fn call_with_function_literal() {
    let program = test_parse("fn(x, y) { x + y }(2, 3)");

    assert_eq!(program.statements.len(), 1);

    let stmt = as_variant!(&program.statements[0], ast::Statement::Expression);

    let expr = as_variant!(&stmt.expression, ast::Expression::Call);

    assert_eq!(expr.args.len(), 2);
    expr_variant!(&expr.args[0], ast::Expression::Integer = 2);
    expr_variant!(&expr.args[1], ast::Expression::Integer = 3);

    let function = as_variant!(&*expr.function, ast::Expression::Function);

    assert_eq!(function.params.len(), 2);
    ident_has_name!(function.params[0], "x");
    ident_has_name!(function.params[1], "y");

    assert_eq!(function.body.statements.len(), 1);

    let body_stmt = as_variant!(&function.body.statements[0], ast::Statement::Expression);

    expr_variant!(
        &body_stmt.expression, Infix => (
            ast::Expression::Identifier = "x",
            token::Token::Plus,
            ast::Expression::Identifier = "y"
        )
    );
}

#[test]
fn function() {
    let program = test_parse("fn(x, y) { x + y; }");

    assert_eq!(program.statements.len(), 1);

    let stmt = as_variant!(&program.statements[0], ast::Statement::Expression);

    let function = as_variant!(&stmt.expression, ast::Expression::Function);

    assert_eq!(function.params.len(), 2);

    ident_has_name!(function.params[0], "x");
    ident_has_name!(function.params[1], "y");

    assert_eq!(function.body.statements.len(), 1);

    let body_stmt = as_variant!(&function.body.statements[0], ast::Statement::Expression);

    expr_variant!(
        &body_stmt.expression, Infix => (
            ast::Expression::Identifier = "x",
            token::Token::Plus,
            ast::Expression::Identifier = "y"
        )
    );
}
#[test]
fn function_params() {
    let tests: [(&str, Vec<&str>); 4] = [
        ("fn() {}", [].into()),
        ("fn(x) {};", ["x"].into()),
        ("fn(x, y) {};", ["x", "y"].into()),
        ("fn(x, y, z) {};", ["x", "y", "z"].into()),
    ];

    for test in tests {
        let program = test_parse(test.0);

        let stmt = as_variant!(&program.statements[0], ast::Statement::Expression);

        let function = as_variant!(&stmt.expression, ast::Expression::Function);

        for (i, exp) in test.1.iter().enumerate() {
            ident_has_name!(function.params[i], *exp);
        }
    }
}

#[test]
fn identifier() {
    let program = test_parse("name;");

    assert_eq!(program.statements.len(), 1);

    let expr = as_variant!(&program.statements[0], ast::Statement::Expression);

    let ident = as_variant!(&expr.expression, ast::Expression::Identifier);

    ident_has_name!(ident, "name");
}

#[test]
fn if_without_else() {
    let program = test_parse("if (x < y) { x }");

    assert_eq!(program.statements.len(), 1);

    let stmt = as_variant!(&program.statements[0], ast::Statement::Expression);

    let if_expr = as_variant!(&stmt.expression, ast::Expression::If);

    assert_eq!(if_expr.token, token::Token::If);

    // testing the condition
    expr_variant!(
        &*if_expr.condition, Infix => (
            ast::Expression::Identifier = "x",
            token::Token::LT,
            ast::Expression::Identifier = "y"
        )
    );

    // testing the consequence block
    let stmt_1 = as_variant!(
        &if_expr.consequence.statements[0],
        ast::Statement::Expression
    );
    expr_variant!(&stmt_1.expression, ast::Expression::Identifier = "x");

    // testing the alternative block
    assert!(if_expr.alternative.is_none());
}

#[test]
fn if_with_else() {
    let program = test_parse("if (x < y) { x } else { y }");

    assert_eq!(program.statements.len(), 1);

    let stmt = as_variant!(&program.statements[0], ast::Statement::Expression);

    let if_expr = as_variant!(&stmt.expression, ast::Expression::If);

    assert_eq!(if_expr.token, token::Token::If);

    // testing the condition
    expr_variant!(
        &*if_expr.condition, Infix => (
            ast::Expression::Identifier = "x",
            token::Token::LT,
            ast::Expression::Identifier = "y"
        )
    );

    // testing the consequence block
    let stmt_0 = as_variant!(
        &if_expr.consequence.statements[0],
        ast::Statement::Expression
    );
    expr_variant!(&stmt_0.expression, ast::Expression::Identifier = "x");

    // testing the alternative block
    let alt = if_expr.alternative.as_ref().expect("alternative is None");
    assert_eq!(alt.token, token::Token::LBrace);

    let stmt_0 = as_variant!(&alt.statements[0], ast::Statement::Expression);
    expr_variant!(&stmt_0.expression, ast::Expression::Identifier = "y");
}

#[test]
fn if_with_multiple_statements() {
    let program = test_parse("if (x < y) { a := 10; x }");

    assert_eq!(program.statements.len(), 1);

    let stmt = as_variant!(&program.statements[0], ast::Statement::Expression);

    let if_expr = as_variant!(&stmt.expression, ast::Expression::If);

    expr_variant!(
        if_expr.condition.as_ref(), Infix => (
            ast::Expression::Identifier = "x",
            token::Token::LT,
            ast::Expression::Identifier = "y"
        )
    );

    assert!(if_expr.alternative.is_none());
    assert_eq!(if_expr.token, token::Token::If);

    // testing consequence block
    let stmt_0 = as_variant!(&if_expr.consequence.statements[0], ast::Statement::Var);
    ident_has_name!(stmt_0.name, "a");
    expr_variant!(&stmt_0.value, ast::Expression::Integer = 10);

    let stmt_1 = as_variant!(
        &if_expr.consequence.statements[1],
        ast::Statement::Expression
    );
    expr_variant!(&stmt_1.expression, ast::Expression::Identifier = "x");
}

#[test]
fn infix() {
    let program = test_parse("1 + 2;");

    assert_eq!(program.statements.len(), 1);

    let expr = as_variant!(&program.statements[0], ast::Statement::Expression);

    expr_variant!(&expr.expression, Infix => (
        ast::Expression::Integer = 1,
        token::Token::Plus,
        ast::Expression::Integer = 2
    ));
}

#[test]
fn integer() {
    let program = test_parse("12;");

    assert_eq!(program.statements.len(), 1);

    let expr = as_variant!(&program.statements[0], ast::Statement::Expression);

    let int = as_variant!(&expr.expression, ast::Expression::Integer);

    assert_eq!(int.token, token::Token::Int("12".into()));
    assert_eq!(int.value, 12);
}

#[test]
fn prefix() {
    let program = test_parse("-12");

    assert_eq!(program.statements.len(), 1);

    let expr = as_variant!(&program.statements[0], ast::Statement::Expression);

    let prefix = as_variant!(&expr.expression, ast::Expression::Prefix);

    assert_eq!(prefix.token, token::Token::Minus);
    assert_eq!(prefix.operator, token::Token::Minus);

    let right = as_variant!(&*prefix.right, ast::Expression::Integer);

    assert_eq!(right.token, token::Token::Int("12".into()));
    assert_eq!(right.value, 12);
}

#[test]
fn string() {
    let program = test_parse("\"Hello, World!\"");

    assert_eq!(program.statements.len(), 1);

    let expr = as_variant!(&program.statements[0], ast::Statement::Expression);

    let s = as_variant!(&expr.expression, ast::Expression::String);

    assert_eq!(s.token, token::Token::String("Hello, World!".into()));
    assert_eq!(s.value, "Hello, World!");
}
