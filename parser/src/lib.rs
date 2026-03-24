use flylang_common::{Address, spanned::Spanned};
use flylang_lexer::token::{Token, TokenValue};
use std::iter::Peekable;

use crate::{ast::ExprKind, error::ParserError, state::ParserState};

pub mod ast;
pub mod error;
pub mod state;

pub type ParserResult<T> = Result<T, ParserError>;

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

    fn peek_address(&mut self) -> Option<Address> {
        self.skip_comments();
        self.tokens.peek().map(|x| x.address.clone())
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

    pub fn parse(&mut self, state: ParserState) -> ParserResult<Vec<ast::Statement>> {
        let mut stmts = Vec::new();

        loop {
            self.skip_whitespaces();

            match self.peek() {
                Some(TokenValue::CloseBrace) if state == ParserState::InBlock => break,
                None => break,
                _ => {}
            }

            stmts.push(self.parse_statement()?);
        }

        Ok(stmts)
    }

    fn parse_block(&mut self) -> ParserResult<ast::Statement> {
        let token_addr = self.expect(TokenValue::OpenBrace).address;

        self.skip_whitespaces();

        let statements = self.parse(ParserState::InBlock)?;

        let end_token_addr = self.expect(TokenValue::CloseBrace).address;

        Ok(ast::Statement::Expr(Spanned {
            value: ast::ExprKind::Block(statements),
            address: token_addr.merge(&end_token_addr),
        }))
    }

    fn parse_func(&mut self) -> ParserResult<ast::Statement> {
        self.expect(TokenValue::Func);

        let name = self.next_token().unwrap();

        if !name.is_identifier() {
            panic!("Name is not an identifier!");
        }

        let name = name.into_spanned_identifier().unwrap();

        eprintln!("Name: {name:?}");

        self.expect(TokenValue::OpenParen);

        let (arguments, addr) = self.parse_argument_list()?;

        let body = self.parse_block()?;

        Ok(ast::Statement::Function {
            name,
            arguments,
            body: Box::new(body),
        })
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
            Spanned {
                value: ast::ExprKind::Number(number_repr),
                address,
            }
        }
    }

    fn parse_argument_list(&mut self) -> ParserResult<(Vec<ast::Expression>, Address)> {
        let mut args = Vec::new();

        let start_address = self.peek_address().unwrap();

        if self.peek() == Some(&TokenValue::CloseParen) {
            self.next_token();

            return Ok((args, start_address.merge(&self.peek_address().unwrap())));
        }

        let end_address: Address;

        loop {
            args.push(self.parse_expression(0)?);

            match self.peek() {
                Some(TokenValue::Comma) => {
                    self.next_token();
                }
                Some(TokenValue::CloseParen) => {
                    end_address = self.next_token().unwrap().address;
                    break;
                }
                other => panic!("expected `,` or `)` in argument list, got {:?}", other),
            }
        }

        Ok((args, start_address.merge(&end_address)))
    }

    fn parse_array_inner(&mut self) -> ParserResult<(Vec<ast::Expression>, Address)> {
        let mut args = Vec::new();

        let start_addr = self.peek_address().unwrap();

        if self.peek() == Some(&TokenValue::CloseBracket) {
            self.next_token();
            return Ok((args, start_addr.merge(&self.peek_address().unwrap())));
        }

        loop {
            args.push(self.parse_expression(0)?);

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

        Ok((args, start_addr.merge(&self.peek_address().unwrap())))
    }

    // Parse an expression.
    // Instead of using recursive descend we use Pratt's parsing method.
    fn parse_expression(&mut self, min_binding_power: usize) -> ParserResult<ast::Expression> {
        self.skip_whitespaces();

        let start_addr = self
            .peek_address()
            .ok_or_else(|| ParserError::UnexpectedEOF)?;

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
            }) => Spanned {
                value: ast::ExprKind::Identifier(nr),
                address,
            },
            // "String"
            Some(Token {
                value: TokenValue::String(nr),
                address,
            }) => Spanned {
                value: ast::ExprKind::String(nr),
                address,
            },
            // -Unary minus
            Some(Token {
                value: TokenValue::Minus,
                address: minus_addr,
            }) => {
                let rhs = self.parse_expression(9)?; // unary minus has high BP

                let merged = minus_addr.merge(&rhs.address);

                Spanned {
                    value: ast::ExprKind::Neg(Box::new(rhs)),
                    address: merged,
                }
            }
            // ['a', 'r', 'r', 'a', 'y']
            Some(Token {
                value: TokenValue::OpenBracket,
                address: bstart,
            }) => {
                let (inner, in_addr) = self.parse_array_inner()?;

                let merged = bstart.merge(&in_addr);

                Spanned {
                    value: ast::ExprKind::Array(inner),
                    address: merged,
                }
            }
            // (Open Paren, ...
            Some(Token {
                value: TokenValue::OpenParen,
                ..
            }) => {
                let inner = self.parse_expression(0)?;
                self.expect(TokenValue::CloseParen);
                inner
            }
            None => todo!("EOF while parsing expression! Handle error gracefully!"),
            value => return Err(ParserError::UnexpectedTokenInExpression { token: value.unwrap() }),
        };

        loop {
            let op = match self.peek() {
                Some(t) => t.clone(),
                None => break,
            };

            // Check for function call
            if op == TokenValue::OpenParen {
                // Magic number: 20 is a high binding power.
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
                let (args, args_addr) = self.parse_argument_list()?;

                lhs = Spanned {
                    value: ast::ExprKind::Call {
                        callee: Box::new(lhs),
                        parameters: args,
                    },
                    address: args_addr,
                };

                continue; // don't fall through to infix handling
            }

            let (left_bp, right_bp) = match op {
                TokenValue::Assign => (1, 2),
                TokenValue::Plus
                | TokenValue::PlusAssign
                | TokenValue::Minus
                | TokenValue::MinusAssign => (4, 5),
                TokenValue::Asterisk
                | TokenValue::Slash
                | TokenValue::RoundingDownDiv
                | TokenValue::RoundingUpDiv
                | TokenValue::Percent => (6, 7),
                TokenValue::Equals
                | TokenValue::Less
                | TokenValue::LessOrEquals
                | TokenValue::Greater
                | TokenValue::GreaterOrEquals => (8, 9),
                TokenValue::OpenBracket => (31, 0),
                TokenValue::Dot => (31, 32),
                _ => break, // not an infix operator
            };

            if left_bp < min_binding_power {
                break; // the outer call has higher claim on this operand
            }

            self.next_token(); // consume the operator
            let rhs = self.parse_expression(right_bp)?;

            let merged = lhs.address.clone().merge(&rhs.address);

            lhs = Spanned {
                value: match op {
                    TokenValue::Plus => ast::ExprKind::Add(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Minus => ast::ExprKind::Sub(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Asterisk => ast::ExprKind::Mul(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Slash => {
                        ast::ExprKind::Div(Box::new(lhs), Box::new(rhs), ast::DivisionKind::Neutral)
                    }
                    TokenValue::RoundingUpDiv => ast::ExprKind::Div(
                        Box::new(lhs),
                        Box::new(rhs),
                        ast::DivisionKind::RoundingUp,
                    ),
                    TokenValue::RoundingDownDiv => ast::ExprKind::Div(
                        Box::new(lhs),
                        Box::new(rhs),
                        ast::DivisionKind::RoundingDown,
                    ),
                    TokenValue::Assign => ast::ExprKind::Assignment {
                        name: Box::new(lhs),
                        value: Box::new(rhs),
                    },
                    TokenValue::Percent => ast::ExprKind::Mod(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Equals => ast::ExprKind::Equals(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Less => ast::ExprKind::Less(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Greater => ast::ExprKind::Greater(Box::new(lhs), Box::new(rhs)),
                    TokenValue::LessOrEquals => {
                        ast::ExprKind::LessOrEquals(Box::new(lhs), Box::new(rhs))
                    }
                    TokenValue::GreaterOrEquals => {
                        ast::ExprKind::GreaterOrEquals(Box::new(lhs), Box::new(rhs))
                    }
                    TokenValue::Dot => ast::ExprKind::PropertyAccess {
                        origin: Box::new(lhs),
                        property: Box::new(rhs),
                    },
                    TokenValue::OpenBracket => {
                        self.expect(TokenValue::CloseBracket);

                        ast::ExprKind::IndexedAccess {
                            origin: Box::new(lhs),
                            index: Box::new(rhs),
                        }
                    }
                    _ => unreachable!(
                        "Maybe you've added a binding power rule, but forgot how to handle them, add new operators."
                    ),
                },
                address: merged,
            }
        }

        Ok(lhs)
    }

    fn parse_return(&mut self) -> ParserResult<ast::Statement> {
        self.next_token();

        let value = self.parse_expression(0)?;

        Ok(ast::Statement::Return {
            value: Box::new(value),
        })
    }

    fn parse_if(&mut self) -> ParserResult<ast::Statement> {
        self.next_token();

        let condition = self.parse_expression(0)?;

        let body = self.parse_block()?;

        let mut else_body: Option<ast::Statement> = None;

        if let Some(&TokenValue::Else) = self.peek() {
            self.next_token();

            if let Some(&TokenValue::If) = self.peek() {
                else_body = Some(self.parse_if()?);
            } else {
                else_body = Some(self.parse_block()?);
            }
        }

        Ok(ast::Statement::If {
            condition: Box::new(condition),
            body: Box::new(body),
            else_body: else_body.map(|x| Box::new(x)),
        })
    }

    fn parse_use(&mut self) -> ParserResult<ast::Statement> {
        self.next_token();

        match self.peek() {
            Some(TokenValue::OpenParen) => {
                let held_value = self.parse_expression(0)?;
                let body = self.parse_block()?;

                Ok(ast::Statement::Scope {
                    held_value: Box::new(held_value),
                    body: Box::new(body),
                })
            }
            _ => {
                let path = self.parse_expression(0)?;

                Ok(ast::Statement::ModuleUsageDeclaration {
                    path: Box::new(path),
                })
            }
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

    fn parse_statement(&mut self) -> ParserResult<ast::Statement> {
        self.skip_whitespaces();

        loop {
            return match self.peek().ok_or_else(|| ParserError::UnexpectedEOF)? {
                TokenValue::Func => Ok(self.parse_func()?),
                TokenValue::If => Ok(self.parse_if()?),
                TokenValue::Return => Ok(self.parse_return()?),
                TokenValue::OpenBrace => Ok(self.parse_block()?),
                TokenValue::Use => Ok(self.parse_use()?),
                tok => {
                    eprintln!("Entering expression with token: {tok:?}");

                    // Parse the left side speculatively as an expression
                    let lhs = self.parse_expression(0)?;

                    // Now decide what kind of statement this is
                    if self.peek() == Some(&TokenValue::Assign) {
                        unreachable!("There was an old code that used to work with assignments. Now, it's moved to expressions parser.");
                        // self.next_token();
                        // let val  = self.parse_expression(0);

                        // eprintln!("Value: {:#?}", lhs.value);

                        // match &lhs.value {
                        //     ExprKind::Identifier(_) => Some(ast::Statement::Expr(Spanned {
                        //         address: lhs.address.clone(),
                        //         value: ast::ExprKind::Assignment {
                        //                 name: Box::new(lhs.clone()),
                        //                 value: Box::new(val),
                        //             }
                        //         }
                        //     )),
                        //     _ => panic!("invalid assignment target"),
                        // }
                    }

                    Ok(ast::Statement::Expr(lhs))
                }
            };
        }
    }
}
