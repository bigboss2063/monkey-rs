use crate::token::Token;

pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

impl Node {
    fn token_literal(&self) -> String {
        match self {
            Node::Program(program) => {
                if program.statements.len() > 0 {
                    return program.statements[0].token_literal();
                } else {
                    return String::from("");
                }
            }
            Node::Statement(statement) => statement.token_literal(),
            Node::Expression(expression) => expression.token_literal(),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Let(Token, Expression, Option<Expression>), // The `x` in `let x = 5`;, is a expression without a value
    Return(Token, Option<Expression>),
    Expression(Token, Option<Expression>), // There are some statements consisting only of expressions, like x + 10;
}

impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Statement::Let(token, _, _) => token.token_literal(),
            Statement::Return(token, _) => token.token_literal(),
            Statement::Expression(token, _) => token.token_literal(),
        }
    }
}

pub enum Expression {
    Identifier(Token, String),
}

impl Expression {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(token, _) => token.token_literal(),
        }
    }
}
