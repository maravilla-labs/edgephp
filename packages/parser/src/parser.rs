// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use crate::{ast::*, error::ParseError, lexer::{Token, lex}};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, token: Token, message: &str) -> Result<(), ParseError> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&token) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::InvalidSyntax {
                message: message.to_string(),
            })
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut items = Vec::new();
        
        while !self.is_at_end() {
            match self.peek() {
                Token::InlineContent(content) => {
                    items.push(ProgramItem::InlineContent(content.clone()));
                    self.advance();
                }
                Token::PhpOpen | Token::PhpShortEcho => {
                    let is_short_echo = matches!(self.peek(), Token::PhpShortEcho);
                    self.advance(); // consume opening tag
                    
                    let mut statements = Vec::new();
                    
                    if is_short_echo {
                        // Short echo tag - parse a single expression and convert to echo
                        let expr = self.parse_expression()?;
                        statements.push(Statement::Echo(vec![expr]));
                        // Don't expect semicolon for short echo tag
                    } else {
                        // Regular PHP block - parse statements until closing tag
                        while !self.is_at_end() && !matches!(self.peek(), Token::PhpClose) {
                            statements.push(self.parse_statement()?);
                        }
                    }
                    
                    // Consume closing tag if present
                    self.match_token(&Token::PhpClose);
                    
                    items.push(ProgramItem::PhpBlock { statements });
                }
                _ => {
                    return Err(ParseError::InvalidSyntax {
                        message: format!("Unexpected token outside PHP tags: {:?}", self.peek()),
                    });
                }
            }
        }
        
        Ok(Program { items })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek() {
            Token::Echo => self.parse_echo(),
            Token::Return => self.parse_return(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Do => self.parse_do_while(),
            Token::For => self.parse_for(),
            Token::Foreach => self.parse_foreach(),
            Token::Switch => self.parse_switch(),
            Token::Function => self.parse_function(),
            Token::Class => self.parse_class(),
            Token::LeftBrace => self.parse_block_statement(),
            Token::Break => {
                self.advance();
                // Semicolon is optional if followed by PhpClose
                if !matches!(self.peek(), Token::PhpClose) {
                    self.consume(Token::Semicolon, "Expected ';' after 'break'")?;
                } else {
                    self.match_token(&Token::Semicolon);
                }
                Ok(Statement::Break)
            }
            Token::Continue => {
                self.advance();
                // Semicolon is optional if followed by PhpClose
                if !matches!(self.peek(), Token::PhpClose) {
                    self.consume(Token::Semicolon, "Expected ';' after 'continue'")?;
                } else {
                    self.match_token(&Token::Semicolon);
                }
                Ok(Statement::Continue)
            }
            Token::Use => self.parse_use(),
            Token::Namespace => self.parse_namespace(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_echo(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'echo'
        
        let mut expressions = vec![self.parse_expression()?];
        
        while self.match_token(&Token::Comma) {
            expressions.push(self.parse_expression()?);
        }
        
        // Semicolon is optional if followed by PhpClose
        if !matches!(self.peek(), Token::PhpClose) {
            self.consume(Token::Semicolon, "Expected ';' after echo statement")?;
        } else {
            // Optional: consume semicolon if present
            self.match_token(&Token::Semicolon);
        }
        Ok(Statement::Echo(expressions))
    }

    fn parse_return(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'return'
        
        let expr = if matches!(self.peek(), Token::Semicolon | Token::PhpClose) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        // Semicolon is optional if followed by PhpClose
        if !matches!(self.peek(), Token::PhpClose) {
            self.consume(Token::Semicolon, "Expected ';' after return statement")?;
        } else {
            // Optional: consume semicolon if present
            self.match_token(&Token::Semicolon);
        }
        Ok(Statement::Return(expr))
    }

    fn parse_if(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'if'
        
        self.consume(Token::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(Token::RightParen, "Expected ')' after if condition")?;
        
        let then_block = self.parse_block_or_statement()?;
        
        // Parse elseif blocks
        let mut elseif_blocks = Vec::new();
        while self.match_token(&Token::ElseIf) {
            self.consume(Token::LeftParen, "Expected '(' after 'elseif'")?;
            let elseif_condition = self.parse_expression()?;
            self.consume(Token::RightParen, "Expected ')' after elseif condition")?;
            let elseif_then_block = self.parse_block_or_statement()?;
            
            elseif_blocks.push(crate::ast::ElseIfBlock {
                condition: elseif_condition,
                then_block: elseif_then_block,
            });
        }
        
        // Parse optional else block
        let else_block = if self.match_token(&Token::Else) {
            Some(self.parse_block_or_statement()?)
        } else {
            None
        };
        
        Ok(Statement::If {
            condition,
            then_block,
            elseif_blocks,
            else_block,
        })
    }

    fn parse_while(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'while'

        self.consume(Token::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(Token::RightParen, "Expected ')' after while condition")?;

        let body = self.parse_block_or_statement()?;

        Ok(Statement::While { condition, body })
    }

    fn parse_do_while(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'do'

        let body = self.parse_block_or_statement()?;

        self.consume(Token::While, "Expected 'while' after do-while body")?;
        self.consume(Token::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(Token::RightParen, "Expected ')' after do-while condition")?;

        // Semicolon is optional if followed by PhpClose
        if !matches!(self.peek(), Token::PhpClose) {
            self.consume(Token::Semicolon, "Expected ';' after do-while statement")?;
        } else {
            self.match_token(&Token::Semicolon);
        }

        Ok(Statement::DoWhile { body, condition })
    }

    fn parse_for(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'for'
        
        self.consume(Token::LeftParen, "Expected '(' after 'for'")?;
        
        // Init
        let init = if matches!(self.peek(), Token::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };
        
        // Condition
        let condition = if matches!(self.peek(), Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.consume(Token::Semicolon, "Expected ';' after for condition")?;
        
        // Update
        let update = if matches!(self.peek(), Token::RightParen) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        self.consume(Token::RightParen, "Expected ')' after for clauses")?;
        
        let body = self.parse_block_or_statement()?;
        
        Ok(Statement::For {
            init,
            condition,
            update,
            body,
        })
    }

    fn parse_foreach(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'foreach'
        
        self.consume(Token::LeftParen, "Expected '(' after 'foreach'")?;
        
        // Parse the array expression
        let array = self.parse_expression()?;
        
        self.consume(Token::As, "Expected 'as' in foreach")?;
        
        // Check if we have key => value or just value
        let (key, value) = if matches!(self.tokens.get(self.current + 1), Some(Token::DoubleArrow)) {
            // We have $key => $value
            let key_var = match self.advance() {
                Token::Variable(k) => k.clone(),
                _ => return Err(ParseError::InvalidSyntax {
                    message: "Expected variable for foreach key".to_string(),
                }),
            };
            self.consume(Token::DoubleArrow, "Expected '=>' after key variable")?;
            let value_var = match self.advance() {
                Token::Variable(v) => v.clone(),
                _ => return Err(ParseError::InvalidSyntax {
                    message: "Expected variable for foreach value".to_string(),
                }),
            };
            (Some(key_var), value_var)
        } else {
            // Just $value
            let value_var = match self.advance() {
                Token::Variable(v) => v.clone(),
                _ => return Err(ParseError::InvalidSyntax {
                    message: "Expected variable for foreach value".to_string(),
                }),
            };
            (None, value_var)
        };
        
        self.consume(Token::RightParen, "Expected ')' after foreach")?;
        
        let body = self.parse_block_or_statement()?;
        
        Ok(Statement::Foreach {
            array,
            key,
            value,
            body,
        })
    }

    fn parse_switch(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'switch'

        self.consume(Token::LeftParen, "Expected '(' after 'switch'")?;
        let expr = self.parse_expression()?;
        self.consume(Token::RightParen, "Expected ')' after switch expression")?;

        self.consume(Token::LeftBrace, "Expected '{' after switch")?;

        let mut cases = Vec::new();

        while !matches!(self.peek(), Token::RightBrace | Token::Eof) {
            match self.peek() {
                Token::Case => {
                    self.advance(); // consume 'case'
                    let value = self.parse_expression()?;
                    self.consume(Token::Colon, "Expected ':' after case value")?;

                    // Parse statements until we hit another case, default, or closing brace
                    let mut statements = Vec::new();
                    while !matches!(self.peek(), Token::Case | Token::Default | Token::RightBrace | Token::Eof) {
                        statements.push(self.parse_statement()?);
                    }

                    cases.push(SwitchCase {
                        value: Some(value),
                        statements,
                    });
                }
                Token::Default => {
                    self.advance(); // consume 'default'
                    self.consume(Token::Colon, "Expected ':' after 'default'")?;

                    // Parse statements until we hit another case, default, or closing brace
                    let mut statements = Vec::new();
                    while !matches!(self.peek(), Token::Case | Token::Default | Token::RightBrace | Token::Eof) {
                        statements.push(self.parse_statement()?);
                    }

                    cases.push(SwitchCase {
                        value: None,
                        statements,
                    });
                }
                _ => {
                    return Err(ParseError::InvalidSyntax {
                        message: "Expected 'case' or 'default' in switch statement".to_string(),
                    });
                }
            }
        }

        self.consume(Token::RightBrace, "Expected '}' after switch cases")?;

        Ok(Statement::Switch { expr, cases })
    }

    fn parse_function(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'function'
        
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err(ParseError::InvalidSyntax {
                message: "Expected function name".to_string(),
            }),
        };
        
        self.consume(Token::LeftParen, "Expected '(' after function name")?;
        
        let mut params = Vec::new();
        if !matches!(self.peek(), Token::RightParen) {
            loop {
                params.push(self.parse_parameter()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        
        let body = self.parse_block()?;
        
        Ok(Statement::Function {
            name,
            params,
            body,
            return_type: None, // TODO: Add type parsing
        })
    }

    fn parse_parameter(&mut self) -> Result<Parameter, ParseError> {
        let name = match self.advance() {
            Token::Variable(n) => n.clone(),
            _ => return Err(ParseError::InvalidSyntax {
                message: "Expected parameter name".to_string(),
            }),
        };
        
        let default = if self.match_token(&Token::Equal) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(Parameter {
            name,
            param_type: None, // TODO: Add type parsing
            default,
        })
    }

    fn parse_class(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'class'
        
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err(ParseError::InvalidSyntax {
                message: "Expected class name".to_string(),
            }),
        };
        
        // TODO: Handle extends and implements
        
        self.consume(Token::LeftBrace, "Expected '{' after class name")?;
        
        let mut members = Vec::new();
        while !matches!(self.peek(), Token::RightBrace) && !self.is_at_end() {
            members.push(self.parse_class_member()?);
        }
        
        self.consume(Token::RightBrace, "Expected '}' after class body")?;
        
        Ok(Statement::Class {
            name,
            extends: None,
            implements: Vec::new(),
            members,
        })
    }

    fn parse_class_member(&mut self) -> Result<ClassMember, ParseError> {
        let visibility = match self.peek() {
            Token::Public => {
                self.advance();
                Visibility::Public
            }
            Token::Private => {
                self.advance();
                Visibility::Private
            }
            Token::Protected => {
                self.advance();
                Visibility::Protected
            }
            _ => Visibility::Public,
        };
        
        if self.match_token(&Token::Function) {
            // Method
            let name = match self.advance() {
                Token::Identifier(n) => n.clone(),
                _ => return Err(ParseError::InvalidSyntax {
                    message: "Expected method name".to_string(),
                }),
            };
            
            self.consume(Token::LeftParen, "Expected '(' after method name")?;
            
            let mut params = Vec::new();
            if !matches!(self.peek(), Token::RightParen) {
                loop {
                    params.push(self.parse_parameter()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            
            self.consume(Token::RightParen, "Expected ')' after parameters")?;
            
            let body = self.parse_block()?;
            
            Ok(ClassMember::Method {
                visibility,
                name,
                params,
                body,
                return_type: None,
            })
        } else {
            // Property
            let name = match self.advance() {
                Token::Variable(n) => n.clone(),
                _ => return Err(ParseError::InvalidSyntax {
                    message: "Expected property name".to_string(),
                }),
            };
            
            let default = if self.match_token(&Token::Equal) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            
            self.consume(Token::Semicolon, "Expected ';' after property")?;
            
            Ok(ClassMember::Property {
                visibility,
                name,
                property_type: None,
                default,
            })
        }
    }

    fn parse_use(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'use'
        
        let mut items = Vec::new();
        
        loop {
            let path = match self.advance() {
                Token::Identifier(p) => p.clone(),
                _ => return Err(ParseError::InvalidSyntax {
                    message: "Expected use path".to_string(),
                }),
            };
            
            // TODO: Handle full namespace paths with \
            
            items.push(UseItem {
                path,
                alias: None,
            });
            
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        
        self.consume(Token::Semicolon, "Expected ';' after use statement")?;
        
        Ok(Statement::Use(UseStatement { items }))
    }

    fn parse_namespace(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume 'namespace'
        
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err(ParseError::InvalidSyntax {
                message: "Expected namespace name".to_string(),
            }),
        };
        
        self.consume(Token::LeftBrace, "Expected '{' after namespace")?;
        
        let mut statements = Vec::new();
        while !matches!(self.peek(), Token::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        self.consume(Token::RightBrace, "Expected '}' after namespace body")?;
        
        Ok(Statement::Namespace(NamespaceStatement {
            name,
            body: Block { statements },
        }))
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.consume(Token::LeftBrace, "Expected '{'")?;
        
        let mut statements = Vec::new();
        while !matches!(self.peek(), Token::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        self.consume(Token::RightBrace, "Expected '}'")?;
        
        Ok(Block { statements })
    }

    fn parse_block_statement(&mut self) -> Result<Statement, ParseError> {
        Ok(Statement::Block(self.parse_block()?))
    }
    
    fn parse_block_or_statement(&mut self) -> Result<Block, ParseError> {
        if matches!(self.peek(), Token::LeftBrace) {
            self.parse_block()
        } else {
            // Single statement without braces
            let stmt = self.parse_statement()?;
            Ok(Block { statements: vec![stmt] })
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expression()?;
        
        // Semicolon is optional if followed by PhpClose
        if !matches!(self.peek(), Token::PhpClose) {
            self.consume(Token::Semicolon, "Expected ';' after expression")?;
        } else {
            // Optional: consume semicolon if present
            self.match_token(&Token::Semicolon);
        }
        Ok(Statement::Expression(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_ternary()?;

        // Check if this is an assignment or compound assignment
        if self.match_token(&Token::Equal) {
            // Make sure left side is assignable
            match &expr {
                Expression::Variable(_) | Expression::ArrayAccess { .. } | Expression::PropertyAccess { .. } => {
                    let right = self.parse_expression()?;
                    Ok(Expression::Assignment {
                        left: Box::new(expr),
                        right: Box::new(right),
                    })
                }
                _ => Err(ParseError::InvalidSyntax {
                    message: "Invalid left-hand side in assignment".to_string(),
                }),
            }
        } else if matches!(self.peek(), Token::PlusEqual | Token::MinusEqual | Token::StarEqual | Token::SlashEqual) {
            // Compound assignment: desugar to regular assignment
            // $a += 5 becomes $a = $a + 5
            match &expr {
                Expression::Variable(_) | Expression::ArrayAccess { .. } | Expression::PropertyAccess { .. } => {
                    let op_token = self.peek().clone();
                    self.advance();

                    // Determine the binary operator
                    let bin_op = match op_token {
                        Token::PlusEqual => BinaryOp::Add,
                        Token::MinusEqual => BinaryOp::Subtract,
                        Token::StarEqual => BinaryOp::Multiply,
                        Token::SlashEqual => BinaryOp::Divide,
                        _ => unreachable!(),
                    };

                    let right = self.parse_expression()?;

                    // Create: $a = $a op right
                    let binary_expr = Expression::Binary {
                        op: bin_op,
                        left: Box::new(expr.clone()),
                        right: Box::new(right),
                    };

                    Ok(Expression::Assignment {
                        left: Box::new(expr),
                        right: Box::new(binary_expr),
                    })
                }
                _ => Err(ParseError::InvalidSyntax {
                    message: "Invalid left-hand side in compound assignment".to_string(),
                }),
            }
        } else {
            Ok(expr)
        }
    }

    fn parse_ternary(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_or()?;
        
        if self.match_token(&Token::Question) {
            let then_expr = self.parse_expression()?;
            self.consume(Token::Colon, "Expected ':' in ternary expression")?;
            let else_expr = self.parse_expression()?;
            
            expr = Expression::Ternary {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            };
        }
        
        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_and()?;
        
        while self.match_token(&Token::Or) {
            let right = self.parse_and()?;
            expr = Expression::Binary {
                op: BinaryOp::Or,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_equality()?;
        
        while self.match_token(&Token::And) {
            let right = self.parse_equality()?;
            expr = Expression::Binary {
                op: BinaryOp::And,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_comparison()?;
        
        loop {
            let op = match self.peek() {
                Token::EqualEqual => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                Token::Identical => BinaryOp::Identical,
                Token::NotIdentical => BinaryOp::NotIdentical,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_concatenation()?;
        
        loop {
            let op = match self.peek() {
                Token::LessThan => BinaryOp::LessThan,
                Token::LessThanEqual => BinaryOp::LessThanOrEqual,
                Token::GreaterThan => BinaryOp::GreaterThan,
                Token::GreaterThanEqual => BinaryOp::GreaterThanOrEqual,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_concatenation()?;
            expr = Expression::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_concatenation(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_addition()?;
        
        while self.match_token(&Token::Dot) {
            let right = self.parse_addition()?;
            expr = Expression::Binary {
                op: BinaryOp::Concat,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_addition(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_multiplication()?;
        
        loop {
            let op = match self.peek() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_multiplication()?;
            expr = Expression::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_unary()?;
        
        loop {
            let op = match self.peek() {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                Token::Percent => BinaryOp::Modulo,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        // Check for cast expression: (int), (string), (bool), (float), (array)
        if matches!(self.peek(), Token::LeftParen) {
            // Look ahead to see if this is a cast
            if self.current + 1 < self.tokens.len() {
                if let Token::Identifier(type_name) = &self.tokens[self.current + 1] {
                    let cast_type = match type_name.as_str() {
                        "int" => Some(Type::Int),
                        "string" => Some(Type::String),
                        "bool" => Some(Type::Bool),
                        "float" => Some(Type::Float),
                        "array" => Some(Type::Array),
                        _ => None,
                    };

                    if let Some(cast_type) = cast_type {
                        // Check if followed by RightParen
                        if self.current + 2 < self.tokens.len()
                            && matches!(self.tokens[self.current + 2], Token::RightParen) {
                            // This is a cast!
                            self.advance(); // consume LeftParen
                            self.advance(); // consume type identifier
                            self.advance(); // consume RightParen

                            let expr = self.parse_unary()?;
                            return Ok(Expression::Cast {
                                cast_type,
                                expr: Box::new(expr),
                            });
                        }
                    }
                }
            }
        }

        match self.peek() {
            Token::Not => {
                self.advance();
                Ok(Expression::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            Token::Minus => {
                self.advance();
                Ok(Expression::Unary {
                    op: UnaryOp::Negate,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            Token::PlusPlus => {
                self.advance();
                Ok(Expression::Unary {
                    op: UnaryOp::PreIncrement,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            Token::MinusMinus => {
                self.advance();
                Ok(Expression::Unary {
                    op: UnaryOp::PreDecrement,
                    expr: Box::new(self.parse_unary()?),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;
        
        loop {
            match self.peek() {
                Token::LeftBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.consume(Token::RightBracket, "Expected ']'")?;
                    expr = Expression::ArrayAccess {
                        array: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Token::Arrow => {
                    self.advance();
                    let token = self.advance().clone();
                    match token {
                        Token::Identifier(method) => {
                            if matches!(self.peek(), Token::LeftParen) {
                                // Method call
                                self.advance();
                                let args = self.parse_arguments()?;
                                self.consume(Token::RightParen, "Expected ')'")?;
                                expr = Expression::MethodCall {
                                    object: Box::new(expr),
                                    method,
                                    args,
                                };
                            } else {
                                // Property access
                                expr = Expression::PropertyAccess {
                                    object: Box::new(expr),
                                    property: method,
                                };
                            }
                        }
                        _ => return Err(ParseError::InvalidSyntax {
                            message: "Expected property or method name".to_string(),
                        }),
                    }
                }
                Token::PlusPlus => {
                    self.advance();
                    expr = Expression::Unary {
                        op: UnaryOp::PostIncrement,
                        expr: Box::new(expr),
                    };
                }
                Token::MinusMinus => {
                    self.advance();
                    expr = Expression::Unary {
                        op: UnaryOp::PostDecrement,
                        expr: Box::new(expr),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.peek() {
            Token::Variable(name) => {
                let var_name = name.clone();
                self.advance();
                Ok(Expression::Variable(var_name))
            }
            Token::Integer(n) => {
                let value = *n;
                self.advance();
                Ok(Expression::Literal(Literal::Integer(value)))
            }
            Token::Float(f) => {
                let value = *f;
                self.advance();
                Ok(Expression::Literal(Literal::Float(value)))
            }
            Token::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expression::Literal(Literal::String(value)))
            }
            Token::InterpolatedString(parts) => {
                let parts_clone = parts.clone();
                self.advance();
                Ok(Expression::Literal(Literal::InterpolatedString(parts_clone)))
            }
            Token::True => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(true)))
            }
            Token::False => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(false)))
            }
            Token::Null => {
                self.advance();
                Ok(Expression::Literal(Literal::Null))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')'")?;
                Ok(expr)
            }
            Token::LeftBracket => {
                self.advance();
                let elements = self.parse_array_elements()?;
                self.consume(Token::RightBracket, "Expected ']'")?;
                Ok(Expression::Array(elements))
            }
            Token::New => {
                self.advance();
                match self.advance() {
                    Token::Identifier(class) => {
                        let class_name = class.clone();
                        self.consume(Token::LeftParen, "Expected '(' after class name")?;
                        let args = self.parse_arguments()?;
                        self.consume(Token::RightParen, "Expected ')'")?;
                        Ok(Expression::New {
                            class: class_name,
                            args,
                        })
                    }
                    _ => Err(ParseError::InvalidSyntax {
                        message: "Expected class name after 'new'".to_string(),
                    }),
                }
            }
            Token::Identifier(name) => {
                let func_name = name.clone();
                self.advance();
                
                if matches!(self.peek(), Token::LeftParen) {
                    self.advance();
                    let args = self.parse_arguments()?;
                    self.consume(Token::RightParen, "Expected ')'")?;
                    Ok(Expression::FunctionCall {
                        name: func_name,
                        args,
                    })
                } else {
                    // This shouldn't happen in PHP, but we'll treat it as a variable reference
                    Ok(Expression::Variable(func_name))
                }
            }
            _ => Err(ParseError::UnexpectedToken {
                position: self.current,
                token: format!("{:?}", self.peek()),
            }),
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut args = Vec::new();
        
        if !matches!(self.peek(), Token::RightParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        Ok(args)
    }

    fn parse_array_elements(&mut self) -> Result<Vec<ArrayElement>, ParseError> {
        let mut elements = Vec::new();
        
        if !matches!(self.peek(), Token::RightBracket) {
            loop {
                let first_expr = self.parse_expression()?;
                
                let element = if self.match_token(&Token::DoubleArrow) {
                    // Key => Value
                    ArrayElement {
                        key: Some(first_expr),
                        value: self.parse_expression()?,
                    }
                } else {
                    // Just value
                    ArrayElement {
                        key: None,
                        value: first_expr,
                    }
                };
                
                elements.push(element);
                
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        Ok(elements)
    }
}

pub fn parse(source: &str) -> Result<Program, ParseError> {
    let tokens = lex(source).map_err(|e| ParseError::LexerError { message: e })?;
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}
