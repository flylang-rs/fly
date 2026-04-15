use flylang_common::{Address, spanned::Spanned, visibility::Visibility};
use flylang_lexer::token::{Token, TokenValue};
use std::iter::Peekable;

use crate::{
    ast::{ExprKind, KeyValueMapWithDuplicates, RecordDefinition, VariableDefinition},
    error::{InvalidArgumentKindDomain, ParserError},
    state::ParserState,
};

// Import tests when necessary
#[cfg(test)]
mod tests;

pub mod ast;
pub mod error;
pub mod state;

pub type ParserResult<T> = Result<T, ParserError>;

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    eof_addr: Address, // for diagnostics
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let eof_addr = tokens.last().map(|x| x.address.clone()).unwrap();

        Self {
            tokens: tokens.into_iter().peekable(),
            eof_addr,
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

    fn peek_whole(&mut self) -> Option<&Token> {
        self.skip_comments();
        self.tokens.peek()
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

    pub fn eof_address(&self) -> &Address {
        &self.eof_addr
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

        let name = self.parse_expression(31)?;

        self.expect(TokenValue::OpenParen);

        let (arguments, _) = self.parse_argument_list()?;

        let body = self.parse_block()?;

        Ok(ast::Statement::Function(ast::Function {
            name: name.into(),
            arguments,
            visibility: Visibility::Global,
            body: Box::new(body),
        }))
    }

    // Maybe it should be in lexer.
    fn check_number(
        &mut self,
        number_repr: String,
        address: Address,
    ) -> ParserResult<ast::Expression> {
        if let Err(_) = number_repr.parse::<f64>() {
            return Err(ParserError::ParsingNumberFailed {
                number: number_repr,
                address,
            });
        } else {
            Ok(Spanned {
                value: ast::ExprKind::Number(number_repr),
                address,
            })
        }
    }

    fn parse_argument_list(&mut self) -> ParserResult<(Vec<ast::Expression>, Address)> {
        let mut args = Vec::new();

        let start_address = self.peek_address().unwrap();

        if self.peek() == Some(&TokenValue::CloseParen) {
            let close = self.next_token();

            return Ok((
                args,
                start_address.merge(
                    &close
                        .map(|x| x.address)
                        .unwrap_or_else(|| self.peek_address().unwrap()),
                ),
            ));
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
        let end_addr: Address;

        if self.peek() == Some(&TokenValue::CloseBracket) {
            // Consume the token and get its address.
            let close = self.next_token();

            return Ok((
                args,
                start_addr.merge(
                    &close
                        .map(|x| x.address)
                        .unwrap_or_else(|| self.peek_address().unwrap()),
                ),
            ));
        }

        loop {
            args.push(self.parse_expression(0)?);

            match self.peek() {
                Some(TokenValue::Comma) => {
                    self.next_token();
                }
                Some(TokenValue::CloseBracket) => {
                    // Consume the token and get its address.
                    let token_addr = self.next_token().map(|x| x.address);

                    end_addr = token_addr.unwrap_or_else(|| self.peek_address().unwrap());

                    break;
                }
                other => panic!("expected `,` or `]` in argument list, got {:?}", other),
            }
        }

        Ok((args, start_addr.merge(&end_addr)))
    }

    // Parse an expression.
    // Instead of using recursive descend we use Pratt's parsing method.
    fn parse_expression(&mut self, min_binding_power: usize) -> ParserResult<ast::Expression> {
        self.skip_whitespaces();

        let start_addr = self
            .peek_address()
            .ok_or_else(|| ParserError::UnexpectedEOF(self.eof_addr.clone()))?;

        let next_token = self
            .next_token()
            .unwrap_or_else(|| todo!("EOF while parsing expression! Handle error gracefully!"));

        let mut lhs = match next_token {
            // nil
            Token {
                value: TokenValue::Nil,
                address,
            } => Spanned {
                value: ast::ExprKind::Nil,
                address,
            },
            // Number
            Token {
                value: TokenValue::Number(nr),
                address,
            } => self.check_number(nr, address)?,
            // Idenitifer
            Token {
                value: TokenValue::Identifier(id),
                address,
            } => Spanned {
                value: ast::ExprKind::Identifier(id),
                address,
            },
            // "String"
            Token {
                value: TokenValue::String(nr),
                address,
            } => Spanned {
                value: ast::ExprKind::String(nr),
                address,
            },
            // true
            Token {
                value: TokenValue::True,
                address,
            } => Spanned {
                value: ast::ExprKind::True,
                address,
            },
            // false
            Token {
                value: TokenValue::False,
                address,
            } => Spanned {
                value: ast::ExprKind::False,
                address,
            },
            // -Unary minus
            Token {
                value: TokenValue::Minus,
                address: minus_addr,
            } => {
                let rhs = self.parse_expression(20)?; // unary minus has high BP

                let merged = minus_addr.merge(&rhs.address);

                Spanned {
                    value: ast::ExprKind::Neg(Box::new(rhs)),
                    address: merged,
                }
            }
            // -Unary minus
            Token {
                value: TokenValue::Bang,
                address: bang_addr,
            } => {
                let rhs = self.parse_expression(20)?; // bang has high BP as minus

                let merged = bang_addr.merge(&rhs.address);

                Spanned {
                    value: ast::ExprKind::Not(Box::new(rhs)),
                    address: merged,
                }
            }
            // ['a', 'r', 'r', 'a', 'y']
            Token {
                value: TokenValue::OpenBracket,
                address: bstart,
            } => {
                let (inner, in_addr) = self.parse_array_inner()?;

                let merged = bstart.merge(&in_addr);

                Spanned {
                    value: ast::ExprKind::Array(inner),
                    address: merged,
                }
            }
            // (Open Paren, ...
            Token {
                value: TokenValue::OpenParen,
                ..
            } => {
                let inner = self.parse_expression(0)?;
                let close = self.expect(TokenValue::CloseParen);
                let merged = start_addr.clone().merge(&close.address);

                Spanned {
                    value: inner.value,
                    address: merged,
                }
            }
            Token {
                value: TokenValue::New,
                ..
            } => {
                let new_obj = self.parse_new_object_declaration()?;

                Spanned::new(ExprKind::New(new_obj.value), new_obj.address)
            }
            value => {
                return Err(ParserError::UnexpectedToken { token: value });
            }
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

            // If we have something like `... : ...` (with colon), it's an anon function!
            // It's a special case, parse it here.
            // Why `min_binding_power < 3`? Because 3 is a binding power that separates assignment and most precedent operation
            // Most precedent operations are `&&` and `||`,
            // So using that condition here will give us (curly braces are a parser scope):
            //   f = { x && 1 }: ...
            //
            // Not:
            //   { f = x && 1 }: ...
            // and
            //    f = x && { 1 }: ...
            // So we can process errors properly.
            // TODO: Move these magic numbers outta here.
            if min_binding_power < 3 && op == TokenValue::Colon {
                self.next_token();

                let rhs = self.parse_expression(0)?;

                let arguments: Vec<ast::Expression> = match &lhs.value {
                    ExprKind::Identifier(_) => {
                        vec![lhs.clone()]
                    }
                    ExprKind::Array(arr) => {
                        for i in arr {
                            if !matches!(i.value, ExprKind::Identifier(_)) {
                                return Err(ParserError::InvalidArgumentKind {
                                    address: i.address.clone(),
                                    domain: InvalidArgumentKindDomain::OnlyId,
                                });
                            }
                        }

                        arr.clone()
                    }
                    _ => {
                        return Err(ParserError::InvalidArgumentKind {
                            address: lhs.address.clone(),
                            domain: InvalidArgumentKindDomain::WholeExpression,
                        });
                    }
                };

                let merged_addr = start_addr.merge(&rhs.address);

                return Ok(Spanned {
                    value: ast::ExprKind::AnonymousFunction {
                        arguments,
                        body: rhs.into(),
                    },
                    address: merged_addr,
                });
            }

            let (left_bp, right_bp) = match op {
                TokenValue::Assign => (1, 2),
                TokenValue::LogicalAnd => (2, 3),
                TokenValue::LogicalOr => (2, 3),
                TokenValue::Ampersand => (3, 4),
                TokenValue::Bar => (3, 4),
                TokenValue::BitShiftLeft => (4, 5),
                TokenValue::BitShiftRight => (4, 5),
                TokenValue::Equals
                | TokenValue::NotEquals
                | TokenValue::Less
                | TokenValue::LessOrEquals
                | TokenValue::Greater
                | TokenValue::GreaterOrEquals => (5, 6),
                TokenValue::Plus
                | TokenValue::PlusAssign
                | TokenValue::Minus
                | TokenValue::MinusAssign => (7, 8),
                TokenValue::Asterisk
                | TokenValue::MulAssign
                | TokenValue::Slash
                | TokenValue::DivAssign
                | TokenValue::RoundingDownDiv
                | TokenValue::RoundingUpDiv
                | TokenValue::RoundingDownDivAssign
                | TokenValue::RoundingUpDivAssign
                | TokenValue::PercentAssign
                | TokenValue::Percent => (9, 10),
                TokenValue::OpenBracket => (31, 0), // suspicious: review and remove it asap
                TokenValue::Dot | TokenValue::PathDelimiter => (31, 32),
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
                    TokenValue::PlusAssign => {
                        ast::ExprKind::AddAssign(Box::new(lhs), Box::new(rhs))
                    }
                    TokenValue::Minus => ast::ExprKind::Sub(Box::new(lhs), Box::new(rhs)),
                    TokenValue::MinusAssign => {
                        ast::ExprKind::SubAssign(Box::new(lhs), Box::new(rhs))
                    }
                    TokenValue::Asterisk => ast::ExprKind::Mul(Box::new(lhs), Box::new(rhs)),
                    TokenValue::MulAssign => ast::ExprKind::MulAssign(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Slash => {
                        ast::ExprKind::Div(Box::new(lhs), Box::new(rhs), ast::DivisionKind::Neutral)
                    }
                    TokenValue::DivAssign => ast::ExprKind::DivAssign(
                        Box::new(lhs),
                        Box::new(rhs),
                        ast::DivisionKind::Neutral,
                    ),
                    TokenValue::RoundingUpDiv => ast::ExprKind::Div(
                        Box::new(lhs),
                        Box::new(rhs),
                        ast::DivisionKind::RoundingUp,
                    ),
                    TokenValue::RoundingUpDivAssign => ast::ExprKind::DivAssign(
                        Box::new(lhs),
                        Box::new(rhs),
                        ast::DivisionKind::RoundingUp,
                    ),
                    TokenValue::RoundingDownDiv => ast::ExprKind::Div(
                        Box::new(lhs),
                        Box::new(rhs),
                        ast::DivisionKind::RoundingDown,
                    ),
                    TokenValue::RoundingDownDivAssign => ast::ExprKind::DivAssign(
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
                    // I thought it can be replaced by Not(Equals(...)), but it needs Spanned to be used.
                    TokenValue::NotEquals => ast::ExprKind::NotEquals(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Less => ast::ExprKind::Less(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Greater => ast::ExprKind::Greater(Box::new(lhs), Box::new(rhs)),
                    TokenValue::LessOrEquals => {
                        ast::ExprKind::LessOrEquals(Box::new(lhs), Box::new(rhs))
                    }
                    TokenValue::GreaterOrEquals => {
                        ast::ExprKind::GreaterOrEquals(Box::new(lhs), Box::new(rhs))
                    }
                    TokenValue::LogicalAnd => ast::ExprKind::And(Box::new(lhs), Box::new(rhs)),
                    TokenValue::LogicalOr => ast::ExprKind::Or(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Ampersand => ast::ExprKind::BitAnd(Box::new(lhs), Box::new(rhs)),
                    TokenValue::Bar => ast::ExprKind::BitOr(Box::new(lhs), Box::new(rhs)),
                    TokenValue::BitShiftLeft => {
                        ast::ExprKind::BitShiftLeft(Box::new(lhs), Box::new(rhs))
                    }
                    TokenValue::BitShiftRight => {
                        ast::ExprKind::BitShiftRight(Box::new(lhs), Box::new(rhs))
                    }
                    TokenValue::Dot => ast::ExprKind::PropertyAccess {
                        origin: Box::new(lhs),
                        property: Box::new(rhs),
                    },
                    TokenValue::PathDelimiter => ast::ExprKind::Path {
                        parent: Box::new(lhs),
                        value: Box::new(rhs),
                    },
                    TokenValue::OpenBracket => {
                        self.expect(TokenValue::CloseBracket);

                        ast::ExprKind::IndexedAccess {
                            origin: Box::new(lhs),
                            index: Box::new(rhs),
                        }
                    }
                    _ => unreachable!(
                        "Maybe you've added a binding power rule, but forgot how to handle them, add new operators. ({:?})",
                        op
                    ),
                },
                address: merged,
            }
        }

        Ok(lhs)
    }

    fn parse_new_object_declaration(&mut self) -> ParserResult<Spanned<ast::NewObjectDeclaration>> {
        let name = self.parse_expression(31)?;

        let (fields, final_addr) = self.parse_new_object_block()?;

        let addr = name.address.clone().merge(&final_addr);

        Ok(Spanned::new(
            ast::NewObjectDeclaration {
                name: name.into(),
                fields,
            },
            addr,
        ))
    }

    fn parse_new_object_block(&mut self) -> ParserResult<(KeyValueMapWithDuplicates, Address)> {
    	self.skip_whitespaces();
    	
        let op_brace_addr = self.expect(TokenValue::OpenBrace).address;

        let mut fields = KeyValueMapWithDuplicates::new();

        loop {
            self.skip_whitespaces();

            let id = self
                .peek_whole()
                .cloned()
                .ok_or_else(|| ParserError::UnexpectedEOF(self.eof_addr.clone()))?;

            let id = match id.value {
                TokenValue::Identifier(val) => Spanned::new(val, id.address),
                TokenValue::CloseBrace => break,
                _ => panic!("Expected identifier, got: {:?}", id.value),
            };

            self.next_token();

            self.expect(TokenValue::Colon);

            let value = self.parse_expression(0)?;

            fields.push((id, value));

            // If we DON'T have a trailing comma, break out
            if !self
                .peek()
                .map(|x| *x == TokenValue::Comma)
                .unwrap_or_default()
            {
                break;
            } else {
                self.next_token();
            }
        }

        self.skip_whitespaces();

        let cl_brace_addr = self.expect(TokenValue::CloseBrace).address;

        Ok((fields, op_brace_addr.merge(&cl_brace_addr)))
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

        Ok(ast::Statement::If(ast::If {
            condition: Box::new(condition),
            body: Box::new(body),
            else_body: else_body.map(Box::new),
        }))
    }

    fn parse_while(&mut self) -> ParserResult<ast::Statement> {
        self.next_token();

        let condition = self.parse_expression(0)?;

        let body = self.parse_block()?;

        Ok(ast::Statement::While(ast::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }))
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

    fn parse_break_or_continue(&mut self) -> ParserResult<ast::Statement> {
        match self.peek() {
            Some(TokenValue::Continue) => {
                self.next_token();

                Ok(ast::Statement::Continue)
            }
            Some(TokenValue::Break) => {
                self.next_token();

                Ok(ast::Statement::Break)
            }
            _ => unreachable!(),
        }
    }

    fn parse_private(&mut self) -> ParserResult<ast::Statement> {
        self.next_token();

        let eof = self.eof_addr.clone();
        let current_token = self.peek().ok_or_else(|| ParserError::UnexpectedEOF(eof))?;

        match current_token {
            TokenValue::Identifier(_) => {
                let expr = self.parse_expression(0)?;

                if let ExprKind::Assignment { name, value } = expr.value {
                    let name_string = name.clone().map(|x| x.as_id().map(|x| x.to_string()));

                    if name_string.value.is_none() {
                        panic!("Expected identifier, got {:?}", &name.value);
                    }

                    let name_string = name_string.map(|x| x.unwrap());

                    return Ok(ast::Statement::VariableDefinition(VariableDefinition {
                        name: name_string,
                        visibility: Visibility::Local,
                        type_annotation: None,
                        value: Some(value),
                    }));
                } else {
                    panic!("Cannot apply `private` to expression `{:?}`", expr.value);
                }
            }
            _ => todo!("Cannot mix `private` with {current_token:?} right now..."),
        }

        unreachable!()
    }

    fn parse_record(&mut self, visibility: Visibility) -> ParserResult<ast::Statement> {
        self.next_token();

        let name = self
            .next_token()
            .ok_or_else(|| ParserError::UnexpectedEOF(self.eof_address().clone()))?;

        let name = match &name.value {
            TokenValue::Identifier(id) => Spanned::new(id.to_string(), name.address),
            _ => return Err(ParserError::UnexpectedToken { token: name }),
        };

        let fields = self.parse_record_block()?;

        Ok(ast::Statement::RecordDefinition(RecordDefinition {
            name,
            visibility,
            fields,
        }))
    }

    fn parse_record_block(&mut self) -> ParserResult<Spanned<Vec<ast::Statement>>> {
        let open_token_addr = self.expect(TokenValue::OpenBrace).address;

        let mut fields: Vec<ast::Statement> = vec![];

        loop {
            self.skip_whitespaces();

            let token = self
                .peek_whole()
                .cloned()
                .ok_or_else(|| ParserError::UnexpectedEOF(self.eof_address().clone()))?;

            match &token.value {
                // Value field declaration
                // `public name`
                // `private name`
                vis @ (TokenValue::Public | TokenValue::Private) => {
                    self.next_token();

                    let name = self
                        .next_token()
                        .ok_or_else(|| ParserError::UnexpectedEOF(self.eof_address().clone()))?;

                    let name = match &name.value {
                        TokenValue::Identifier(id) => {
                            Spanned::new(id.clone(), name.address.clone())
                        }
                        _ => return Err(ParserError::UnexpectedToken { token: name }),
                    };

                    // eprintln!("Public or private field with value: {:?}", name);

                    fields.push(ast::Statement::VariableDefinition(VariableDefinition {
                        name,
                        visibility: match vis {
                            TokenValue::Public => Visibility::Global,
                            TokenValue::Private => Visibility::Local,
                            _ => unreachable!(),
                        },
                        type_annotation: None, // TODO: Parse type annotation
                        value: None,           // it's just a declaration
                    }));
                }
                // Closing brace
                TokenValue::CloseBrace => break,
                // Anything else
                _ => return Err(ParserError::UnexpectedToken { token: token }),
            };
        }

        let close_token_addr = self.expect(TokenValue::CloseBrace).address;

        Ok(Spanned::new(
            fields,
            open_token_addr.merge(&close_token_addr),
        ))
    }

    fn parse_statement(&mut self) -> ParserResult<ast::Statement> {
        self.skip_whitespaces();

        let eof = self.eof_addr.clone();

        loop {
            return match self.peek().ok_or_else(|| ParserError::UnexpectedEOF(eof))? {
                TokenValue::Func => Ok(self.parse_func()?),
                TokenValue::If => Ok(self.parse_if()?),
                TokenValue::While => Ok(self.parse_while()?),
                TokenValue::Return => Ok(self.parse_return()?),
                TokenValue::OpenBrace => Ok(self.parse_block()?),
                TokenValue::Break | TokenValue::Continue => Ok(self.parse_break_or_continue()?),
                TokenValue::Use => Ok(self.parse_use()?),
                TokenValue::Private => Ok(self.parse_private()?),
                TokenValue::Record => Ok(self.parse_record(Visibility::Global)?),
                _ /* tok */ => {
                    let lhs = self.parse_expression(0)?;

                    Ok(ast::Statement::Expr(lhs))
                }
            };
        }
    }
}
