// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<ProgramItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProgramItem {
    PhpBlock {
        statements: Vec<Statement>,
    },
    InlineContent(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Expression(Expression),
    Return(Option<Expression>),
    If {
        condition: Expression,
        then_block: Block,
        elseif_blocks: Vec<ElseIfBlock>,
        else_block: Option<Block>,
    },
    While {
        condition: Expression,
        body: Block,
    },
    DoWhile {
        body: Block,
        condition: Expression,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Expression>,
        body: Block,
    },
    Function {
        name: String,
        params: Vec<Parameter>,
        body: Block,
        return_type: Option<Type>,
    },
    Class {
        name: String,
        extends: Option<String>,
        implements: Vec<String>,
        members: Vec<ClassMember>,
    },
    Echo(Vec<Expression>),
    Block(Block),
    Break,
    Continue,
    Use(UseStatement),
    Namespace(NamespaceStatement),
    Foreach {
        array: Expression,
        key: Option<String>,
        value: String,
        body: Block,
    },
    Switch {
        expr: Expression,
        cases: Vec<SwitchCase>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchCase {
    pub value: Option<Expression>,  // None for default case
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElseIfBlock {
    pub condition: Expression,
    pub then_block: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Variable(String),
    Literal(Literal),
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    Assignment {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    MethodCall {
        object: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
    PropertyAccess {
        object: Box<Expression>,
        property: String,
    },
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    Array(Vec<ArrayElement>),
    New {
        class: String,
        args: Vec<Expression>,
    },
    Ternary {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
    Cast {
        cast_type: Type,
        expr: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    InterpolatedString(Vec<InterpolatedPart>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterpolatedPart {
    Text(String),
    Variable(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Identical,
    NotIdentical,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Concat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Negate,
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<Type>,
    pub default: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Array,
    Object(String),
    Mixed,
    Void,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClassMember {
    Property {
        visibility: Visibility,
        name: String,
        property_type: Option<Type>,
        default: Option<Expression>,
    },
    Method {
        visibility: Visibility,
        name: String,
        params: Vec<Parameter>,
        body: Block,
        return_type: Option<Type>,
    },
    Constructor {
        visibility: Visibility,
        params: Vec<Parameter>,
        body: Block,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayElement {
    pub key: Option<Expression>,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseStatement {
    pub items: Vec<UseItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseItem {
    pub path: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamespaceStatement {
    pub name: String,
    pub body: Block,
}
