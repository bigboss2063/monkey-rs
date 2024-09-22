use std::any::{Any, TypeId};

use crate::{
    ast::{Expression, Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let parser = Self {
            lexer,
            cur_token,
            peek_token,
            errors: vec![],
        };
        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        // Loop until we reach the end of the tokens
        while !self.cur_token_is("EOF") {
            match self.parse_statement() {
                Some(statement) => {
                    statements.push(statement);
                }
                None => {}
            }
            self.next_token();
        }

        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let let_token = self.cur_token.clone();

        // The token after LET is the IDENT
        if !self.expect_peek("IDENT") {
            return None;
        }

        let assign_token = self.cur_token.clone();
        let assign_token_literal = assign_token.token_literal();

        // The token after IDENT is the ASSIGN
        if !self.expect_peek("ASSIGN") {
            return None;
        }

        // Todo Handle expressions
        while !self.cur_token_is("SEMICOLON") {
            self.next_token();
        }

        Some(Statement::Let(
            let_token,
            Expression::Identifier(assign_token, assign_token_literal),
            None,
        ))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let return_token = self.cur_token.clone();

        self.next_token();

        // Todo Handle expressions
        while !self.cur_token_is("SEMICOLON") {
            self.next_token();
        }

        Some(Statement::Return(return_token, None))
    }

    fn cur_token_is(&self, expect_token_name: &str) -> bool {
        self.cur_token.token_name() == expect_token_name
    }

    fn peek_token_is(&self, expect_token_name: &str) -> bool {
        self.peek_token.token_name() == expect_token_name
    }

    fn expect_peek(&mut self, expect_token_name: &str) -> bool {
        if self.peek_token_is(expect_token_name) {
            self.next_token();
            true
        } else {
            self.peek_error(expect_token_name);
            false
        }
    }

    fn peek_error(&mut self, expect_token_name: &str) {
        let msg = format!(
            "Expected '{}' but found '{}'",
            expect_token_name,
            self.peek_token.token_name(),
        );
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod tests {

    use crate::{ast::Statement, lexer::Lexer};

    use super::Parser;

    #[test]
    fn test_let_statement() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 114514;
        ";

        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);
        check_parser_errors(&parser, 0);

        let test_cases = vec!["x", "y", "foobar"];

        for (i, &test_case) in test_cases.iter().enumerate() {
            let statement = &program.statements[i];
            assert_eq!(statement.token_literal(), "let");
            match statement {
                Statement::Let(_, name, _) => {
                    assert_eq!(name.token_literal(), test_case);
                }
                _ => {}
            };
        }
    }

    #[test]
    fn test_let_statement_errors() {
        let input = "
        let x 5;
        let = 10;
        let 114514;
        ";

        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);

        let _ = parser.parse_program();

        check_parser_errors(&parser, 3);
    }

    #[test]
    fn test_return_statement() {
        let input = "
        return 5;
        return 10;
        return 993;
        ";

        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);
        check_parser_errors(&parser, 0);

        let test_cases = vec!["return", "return", "return"];

        for (i, &test_case) in test_cases.iter().enumerate() {
            let statement = &program.statements[i];
            assert_eq!(statement.token_literal(), test_case);
        }
    }

    fn check_parser_errors(parser: &Parser, count: usize) {
        if parser.errors.len() != count {
            assert!(
                false,
                "parser should have {} errors, but found {}",
                count,
                parser.errors.len()
            );
        }

        println!("Parser has {} errors:", parser.errors.len());
        for (i, error) in parser.errors.iter().enumerate() {
            println!("[{}]: {}", i + 1, error);
        }
    }
}
