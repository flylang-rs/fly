use flylang_common::{Address, spanned::Spanned};
use flylang_lexer::token::{Token, TokenValue};
use std::iter::Peekable;

use crate::state::ParserState;

pub mod ast;
pub mod state;

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn skip_comments(&mut self) {
        while matches!(
            self.tokens.peek().map(|t| &t.value),
            Some(TokenValue::Comment(_))
        ) {
            self.tokens.next();
        }
    }

    fn peek(&mut self) -> Option<&TokenValue> {
        self.skip_comments();
        self.tokens.peek().map(|t| &t.value)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        loop {
            let token = self.tokens.peek()?;

            if matches!(&token.value, &TokenValue::Comment(_)) {
                self.tokens.next();

                continue;
            }

            return self.tokens.next();
        }
    }

    fn expect(&mut self, expected: TokenValue) -> Token {
        match self.tokens.next() {
            Some(token) if token.value == expected => token,
            Some(token) => {
                panic!("expected {:?}, got {:?}", expected, token.value);
            }
            None => {
                panic!("expected {:?}, got end of input", expected);
            }
        }
    }

    pub fn parse(&mut self, state: ParserState) -> Vec<ast::Statement> {
        let mut stmts = Vec::new();

        loop {
            self.skip_whitespaces();

            match self.peek() {
                Some(TokenValue::CloseBrace) if state == ParserState::InBlock => break,
                None => break,
                _ => {}
            }

            if let Some(stmt) = self.parse_statement() {
                stmts.push(stmt);
            }
        }

        stmts
    }

    fn parse_block(&mut self) -> ast::Statement {
        self.expect(TokenValue::OpenBrace);
        self.skip_whitespaces();

        let statements = self.parse(ParserState::InBlock);

        self.expect(TokenValue::CloseBrace);

        ast::Statement::Expr(ast::Expression::Block(statements))
    }

    fn parse_func(&mut self) -> ast::Statement {
        self.expect(TokenValue::Func);

        let name = self.next_token().unwrap();

        if !name.is_identifier() {
            panic!("Name is not an identifier!");
        }

        let name = name.into_spanned_identifier().unwrap();

        eprintln!("Name: {name:?}");

        self.expect(TokenValue::OpenParen);

        let arguments = self.parse_argument_list();

        let body = self.parse_block();

        ast::Statement::Function {
            name,
            arguments,
            body: Box::new(body),
        }
    }

    // Maybe it should be in lexer.
    fn check_number(&mut self, number_repr: String, address: Address) -> ast::Expression {
        if let Err(_) = number_repr.parse::<usize>() {
            // Error
            let (line, col) = address.source.location(address.span.start);

            panic!(
                "Failed to parse a number: {number_repr} ({}:{}:{})",
                address.source.filepath, line, col
            );
        } else {
            ast::Expression::Number(Spanned {
                value: number_repr,
                address,
            })
        }
    }

    fn parse_argument_list(&mut self) -> Vec<ast::Expression> {
        let mut args = Vec::new();

        if self.peek() == Some(&TokenValue::CloseParen) {
            self.next_token();
            return args;
        }

        loop {
            args.push(self.parse_expression(0));

            match self.peek() {
                Some(TokenValue::Comma) => {
                    self.next_token();
                }
                Some(TokenValue::CloseParen) => {
                    self.next_token();
                    break;
                }
                other => panic!("expected `,` or `)` in argument list, got {:?}", other),
            }
        }

        args
    }

    fn parse_array_inner(&mut self) -> Vec<ast::Expression> {
        let mut args = Vec::new();

        if self.peek() == Some(&TokenValue::CloseBracket) {
            self.next_token();
            return args;
        }

        loop {
            args.push(self.parse_expression(0));

            match self.peek() {
                Some(TokenValue::Comma) => {
                    self.next_token();
                }
                Some(TokenValue::CloseBracket) => {
                    self.next_token();
                    break;
                }
                other => panic!("expected `,` or `]` in argument list, got {:?}", other),
            }
        }

        args
    }

    // Parse an expression.
    // Instead of using recursive descend we use Pratt's parsing method.
    fn parse_expression(&mut self, min_binding_power: usize) -> ast::Expression {
        self.skip_whitespaces();

        let mut lhs = match self.next_token() {
            // Number
            Some(Token {
                value: TokenValue::Number(nr),
                address,
            }) => self.check_number(nr, address),
            // Idenitifer
            Some(Token {
                value: TokenValue::Identifier(nr),
                address,
            }) => ast::Expression::Identifier(Spanned { value: nr, address }),
            // "String"
            Some(Token {
                value: TokenValue::String(nr),
                address,
            }) => ast::Expression::String(Spanned { value: nr, address }),
            // -Unary minus
            Some(Token {
                value: TokenValue::Minus,
                ..
            }) => {
                let rhs = self.parse_expression(9); // unary minus has high BP
                ast::Expression::Neg(Box::new(rhs))
            }
            // ['a', 'r', 'r', 'a', 'y']
            Some(Token {
                value: TokenValue::OpenBracket,
                ..
            }) => {
                let inner = self.parse_array_inner();
                ast::Expression::Array(inner)
            }
            // (Open Paren, ...
            Some(Token {
                value: TokenValue::OpenParen,
                ..
            }) => {
                let inner = self.parse_expression(0);
                self.expect(TokenValue::CloseParen);
                inner
            }

            value => todo!("Got unexpected token: {value:?}"),
        };

        loop {
            let op = match self.peek() {
                Some(t) => t.clone(),
                None => break,
            };

            // Check for function call
            if op == TokenValue::OpenParen {
                // Magic number: 20 is a highest binding power.
                // By using it, token list [foo, (, x, ), +, 1] will be unrolled into
                // | foo
                // | `- (x)
                // | +
                // | 1
                //
                // Not into:
                // | foo
                // | `- (x)
                // | `- +
                // | `- 1

                if 20 < min_binding_power {
                    break;
                }

                self.next_token(); // consume `(`
                let args = self.parse_argument_list();

                lhs = ast::Expression::Call {
                    callee: Box::new(lhs),
                    parameters: args,
                };

                continue; // don't fall through to infix handling
            }

            let (left_bp, right_bp) = match op {
                TokenValue::Assign => (1, 2),
                TokenValue::Plus | TokenValue::Minus => (3, 4),
                TokenValue::Asterisk
                | TokenValue::Slash
                | TokenValue::RoundingDownDiv
                | TokenValue::RoundingUpDiv
                | TokenValue::Percent => (5, 6),
                TokenValue::Equals
                | TokenValue::Less
                | TokenValue::LessOrEquals
                | TokenValue::Greater
                | TokenValue::GreaterOrEquals => (7, 8),
                _ => break, // not an infix operator
            };

            if left_bp < min_binding_power {
                break; // the outer call has higher claim on this operand
            }

            self.next_token(); // consume the operator
            let rhs = self.parse_expression(right_bp);

            lhs = match op {
                TokenValue::Plus => ast::Expression::Add(Box::new(lhs), Box::new(rhs)),
                TokenValue::Minus => ast::Expression::Sub(Box::new(lhs), Box::new(rhs)),
                TokenValue::Asterisk => ast::Expression::Mul(Box::new(lhs), Box::new(rhs)),
                TokenValue::Slash => {
                    ast::Expression::Div(Box::new(lhs), Box::new(rhs), ast::DivisionKind::Neutral)
                }
                TokenValue::RoundingUpDiv => ast::Expression::Div(
                    Box::new(lhs),
                    Box::new(rhs),
                    ast::DivisionKind::RoundingUp,
                ),
                TokenValue::RoundingDownDiv => ast::Expression::Div(
                    Box::new(lhs),
                    Box::new(rhs),
                    ast::DivisionKind::RoundingDown,
                ),
                TokenValue::Assign => ast::Expression::Assignment { name: Box::new(lhs), value: Box::new(rhs) },
                TokenValue::Percent => ast::Expression::Mod(Box::new(lhs), Box::new(rhs)),
                TokenValue::Equals => ast::Expression::Equals(Box::new(lhs), Box::new(rhs)),
                TokenValue::Less => ast::Expression::Less(Box::new(lhs), Box::new(rhs)),
                TokenValue::Greater => ast::Expression::Greater(Box::new(lhs), Box::new(rhs)),
                TokenValue::LessOrEquals => {
                    ast::Expression::LessOrEquals(Box::new(lhs), Box::new(rhs))
                }
                TokenValue::GreaterOrEquals => {
                    ast::Expression::GreaterOrEquals(Box::new(lhs), Box::new(rhs))
                }
                _ => unreachable!(
                    "Maybe you've added a binding power rule, but forgot how to handle them, add new operators."
                ),
            };
        }

        lhs
    }

    fn parse_return(&mut self) -> ast::Statement {
        self.next_token();

        let value = self.parse_expression(0);

        ast::Statement::Return {
            value: Box::new(value),
        }
    }

    fn parse_if(&mut self) -> ast::Statement {
        self.next_token();

        let condition = self.parse_expression(0);

        let body = self.parse_block();

        let mut else_body: Option<ast::Statement> = None;

        if let Some(&TokenValue::Else) = self.peek() {
            self.next_token();

            if let Some(&TokenValue::If) = self.peek() {
                else_body = Some(self.parse_if());
            } else {
                else_body = Some(self.parse_block());
            }
        }

        ast::Statement::If {
            condition: Box::new(condition),
            body: Box::new(body),
            else_body: else_body.map(|x| Box::new(x)),
        }
    }

    fn skip_whitespaces(&mut self) {
        loop {
            match self.peek() {
                Some(TokenValue::Newline | TokenValue::Semicolon) => {
                    self.next_token();
                }
                _ => break,
            }
        }
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        self.skip_whitespaces();

        loop {
            return match self.peek()? {
                TokenValue::Func => Some(self.parse_func()),
                TokenValue::If => Some(self.parse_if()),
                TokenValue::Return => Some(self.parse_return()),
                TokenValue::OpenBrace => Some(self.parse_block()),
                tok => {
                    eprintln!("Entering expression with token: {tok:?}");

                    // Parse the left side speculatively as an expression
                    let lhs = self.parse_expression(0);

                    // Now decide what kind of statement this is
                    if self.peek() == Some(&TokenValue::Assign) {
                        self.next_token();
                        let value = self.parse_expression(0);

                        match lhs {
                            id @ ast::Expression::Identifier(_) => {
                                Some(ast::Statement::Expr(ast::Expression::Assignment {
                                    name: Box::new(id),
                                    value: Box::new(value),
                                }))
                            }
                            _ => panic!("invalid assignment target"),
                        }
                    } else {
                        Some(ast::Statement::Expr(lhs))
                    }
                }
            };
        }
    }
}
