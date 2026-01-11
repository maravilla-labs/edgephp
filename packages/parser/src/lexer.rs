// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1, take_until},
    character::complete::{char, multispace1, digit1},
    combinator::{map, opt, recognize, value},
    sequence::{delimited, pair, preceded, tuple},
};
use crate::ast::InterpolatedPart;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    InterpolatedString(Vec<InterpolatedPart>),
    True,
    False,
    Null,
    
    // Identifiers and Keywords
    Identifier(String),
    Variable(String),
    
    // Keywords
    If,
    Else,
    ElseIf,
    While,
    Do,
    For,
    Foreach,
    As,
    Function,
    Class,
    Public,
    Private,
    Protected,
    Return,
    Echo,
    New,
    Use,
    Namespace,
    Break,
    Continue,
    Switch,
    Case,
    Default,

    // Operators
    Plus,
    Minus,
    PlusPlus,
    MinusMinus,
    Star,
    Slash,
    Percent,
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    EqualEqual,
    NotEqual,
    Identical,
    NotIdentical,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    And,
    Or,
    Not,
    Dot,
    Arrow,
    DoubleArrow,
    Question,
    Colon,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    
    // Special
    PhpOpen,
    PhpClose,
    PhpShortEcho,
    InlineContent(String),
    Eof,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut remaining = input;
    let mut in_php = false;
    
    while !remaining.is_empty() {
        if !in_php {
            // Look for PHP opening tag or short echo tag at the current position
            if remaining.starts_with("<?=") {
                // Found short echo tag at current position
                tokens.push(Token::PhpShortEcho);
                remaining = &remaining[3..];
                in_php = true;
            } else if remaining.starts_with("<?php") {
                // Found full PHP tag at current position
                tokens.push(Token::PhpOpen);
                remaining = &remaining[5..];
                // Check for required whitespace after <?php
                if remaining.is_empty() || !remaining.chars().next().unwrap().is_whitespace() {
                    return Err("Whitespace required after <?php".to_string());
                }
                in_php = true;
            } else if remaining.starts_with("<?") {
                // Found short tag at current position
                tokens.push(Token::PhpOpen);
                remaining = &remaining[2..];
                in_php = true;
            } else {
                // Look for the next PHP tag
                let mut end_pos = remaining.len();
                
                // Find the nearest PHP tag
                if let Some(pos) = remaining.find("<?=") {
                    end_pos = end_pos.min(pos);
                }
                if let Some(pos) = remaining.find("<?php") {
                    end_pos = end_pos.min(pos);
                }
                if let Some(pos) = remaining.find("<?") {
                    // Make sure this is not part of <?php or <?=
                    if pos + 1 < remaining.len() {
                        let next_char = remaining.chars().nth(pos + 1).unwrap();
                        if next_char != 'p' && next_char != '=' {
                            end_pos = end_pos.min(pos);
                        }
                    } else {
                        end_pos = end_pos.min(pos);
                    }
                }
                
                // Add inline content up to the next PHP tag (or end of string)
                if end_pos > 0 {
                    tokens.push(Token::InlineContent(remaining[..end_pos].to_string()));
                    remaining = &remaining[end_pos..];
                } else {
                    // This shouldn't happen, but just in case
                    remaining = &remaining[1..];
                }
            }
        } else {
            // Inside PHP code
            // Skip whitespace
            if let Ok((rest, _)) = multispace1::<_, nom::error::Error<_>>(remaining) {
                remaining = rest;
                continue;
            }
            
            // Skip comments
            if let Ok((rest, _)) = comment(remaining) {
                remaining = rest;
                continue;
            }
            
            // Check for PHP closing tag
            if remaining.starts_with("?>") {
                tokens.push(Token::PhpClose);
                remaining = &remaining[2..];
                in_php = false;
                continue;
            }
            
            // Try to match tokens
            match lex_token(remaining) {
                Ok((rest, token)) => {
                    tokens.push(token);
                    remaining = rest;
                }
                Err(_) => {
                    return Err(format!("Unable to parse token at: {}", &remaining[..20.min(remaining.len())]));
                }
            }
        }
    }
    
    tokens.push(Token::Eof);
    Ok(tokens)
}

fn lex_token(input: &str) -> IResult<&str, Token> {
    alt((
        // Keywords and identifiers
        keyword_or_identifier,
        
        // Variables
        variable,
        
        // Numbers
        number,
        
        // Strings
        string_literal,
        
        // Operators and delimiters
        operators,
        delimiters,
    ))(input)
}


fn comment(input: &str) -> IResult<&str, &str> {
    alt((
        // Single line comment with //
        recognize(pair(tag("//"), take_while(|c| c != '\n'))),
        // Single line comment with #
        recognize(pair(tag("#"), take_while(|c| c != '\n'))),
        // Multi-line comment
        recognize(delimited(tag("/*"), take_until("*/"), tag("*/"))),
    ))(input)
}

fn keyword_or_identifier(input: &str) -> IResult<&str, Token> {
    let (input, word) = identifier_str(input)?;
    
    let token = match word {
        "if" => Token::If,
        "else" => Token::Else,
        "elseif" => Token::ElseIf,
        "while" => Token::While,
        "do" => Token::Do,
        "for" => Token::For,
        "foreach" => Token::Foreach,
        "as" => Token::As,
        "function" => Token::Function,
        "class" => Token::Class,
        "public" => Token::Public,
        "private" => Token::Private,
        "protected" => Token::Protected,
        "return" => Token::Return,
        "echo" => Token::Echo,
        "new" => Token::New,
        "use" => Token::Use,
        "namespace" => Token::Namespace,
        "break" => Token::Break,
        "continue" => Token::Continue,
        "switch" => Token::Switch,
        "case" => Token::Case,
        "default" => Token::Default,
        "true" => Token::True,
        "false" => Token::False,
        "null" => Token::Null,
        _ => Token::Identifier(word.to_string()),
    };
    
    Ok((input, token))
}

fn identifier_str(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            take_while1(|c: char| c.is_alphabetic() || c == '_'),
            take_while(|c: char| c.is_alphanumeric() || c == '_')
        )
    )(input)
}

fn variable(input: &str) -> IResult<&str, Token> {
    map(
        preceded(char('$'), identifier_str),
        |name| Token::Variable(name.to_string())
    )(input)
}

fn number(input: &str) -> IResult<&str, Token> {
    alt((
        // Float
        map(
            recognize(tuple((
                opt(char('-')),
                digit1,
                char('.'),
                digit1
            ))),
            |s: &str| Token::Float(s.parse().unwrap())
        ),
        // Integer
        map(
            recognize(pair(opt(char('-')), digit1)),
            |s: &str| Token::Integer(s.parse().unwrap())
        ),
    ))(input)
}

fn string_literal(input: &str) -> IResult<&str, Token> {
    alt((
        // Double-quoted string - handle escape sequences and interpolation
        map(
            delimited(
                char('"'),
                interpolated_string('"'),
                char('"')
            ),
            |parts: Vec<InterpolatedPart>| {
                // If there's only one part and it's text, return a simple string
                if parts.len() == 1 {
                    if let InterpolatedPart::Text(s) = &parts[0] {
                        return Token::String(s.clone());
                    }
                }
                Token::InterpolatedString(parts)
            }
        ),
        // Single-quoted string - only handle \' and \\
        map(
            delimited(
                char('\''),
                single_quoted_string(),
                char('\'')
            ),
            |s: String| Token::String(s)
        ),
    ))(input)
}

fn interpolated_string(quote: char) -> impl Fn(&str) -> IResult<&str, Vec<InterpolatedPart>> {
    move |input: &str| {
        let mut parts = Vec::new();
        let mut current_text = String::new();
        let mut chars = input.char_indices();
        
        while let Some((idx, ch)) = chars.next() {
            if ch == quote {
                // Found closing quote
                if !current_text.is_empty() {
                    parts.push(InterpolatedPart::Text(current_text));
                }
                return Ok((&input[idx..], parts));
            } else if ch == '\\' {
                // Handle escape sequences
                if let Some((_, escaped)) = chars.next() {
                    match escaped {
                        'n' => current_text.push('\n'),
                        't' => current_text.push('\t'),
                        'r' => current_text.push('\r'),
                        '\\' => current_text.push('\\'),
                        '"' => current_text.push('"'),
                        '\'' => current_text.push('\''),
                        '$' => current_text.push('$'), // Escaped dollar sign
                        _ => {
                            current_text.push('\\');
                            current_text.push(escaped);
                        }
                    }
                }
            } else if ch == '$' {
                // Check if this is a variable or escaped dollar
                if let Some((var_start_idx, next_ch)) = chars.next() {
                    if next_ch == '$' {
                        // Double dollar sign - escaped dollar
                        current_text.push('$');
                    } else if next_ch.is_alphabetic() || next_ch == '_' {
                        // This is a variable - save current text first
                        if !current_text.is_empty() {
                            parts.push(InterpolatedPart::Text(current_text));
                            current_text = String::new();
                        }
                        
                        // Collect the variable name
                        let mut var_name = String::new();
                        var_name.push(next_ch);
                        
                        // Continue collecting alphanumeric characters
                        let mut found_end = false;
                        let mut end_char = '\0';
                        let mut continue_from = var_start_idx + 1;
                        
                        while let Some((idx, ch)) = chars.next() {
                            if ch.is_alphanumeric() || ch == '_' {
                                var_name.push(ch);
                                continue_from = idx + 1;
                            } else {
                                // End of variable name
                                found_end = true;
                                end_char = ch;
                                continue_from = idx;
                                break;
                            }
                        }
                        
                        // Add the variable
                        parts.push(InterpolatedPart::Variable(var_name));
                        
                        // Check if we found the end
                        if found_end {
                            if end_char == quote {
                                // If we hit the closing quote, return
                                return Ok((&input[continue_from..], parts));
                            } else if end_char == '\\' {
                                // Handle escape sequence after variable
                                if let Some((_, escaped)) = chars.next() {
                                    match escaped {
                                        'n' => current_text.push('\n'),
                                        't' => current_text.push('\t'),
                                        'r' => current_text.push('\r'),
                                        '\\' => current_text.push('\\'),
                                        '"' => current_text.push('"'),
                                        '\'' => current_text.push('\''),
                                        '$' => current_text.push('$'),
                                        _ => {
                                            current_text.push('\\');
                                            current_text.push(escaped);
                                        }
                                    }
                                }
                            } else if end_char == '$' {
                                // Another variable immediately after - reprocess from this position
                                // We need to continue parsing from the $ character
                                let remaining = &input[continue_from..];
                                match interpolated_string(quote)(remaining) {
                                    Ok((rest, mut more_parts)) => {
                                        // Add any text we accumulated
                                        if !current_text.is_empty() {
                                            parts.push(InterpolatedPart::Text(current_text));
                                        }
                                        // Add the additional parts
                                        parts.append(&mut more_parts);
                                        return Ok((rest, parts));
                                    }
                                    Err(e) => return Err(e),
                                }
                            } else {
                                // Otherwise, continue processing from this character
                                current_text.push(end_char);
                            }
                        }
                    } else {
                        // Not a variable, just a dollar sign
                        current_text.push('$');
                        current_text.push(next_ch);
                    }
                } else {
                    // Dollar sign at end of string
                    current_text.push('$');
                }
            } else {
                current_text.push(ch);
            }
        }
        
        Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Eof)))
    }
}

fn single_quoted_string() -> impl Fn(&str) -> IResult<&str, String> {
    |input: &str| {
        let mut result = String::new();
        let mut chars = input.char_indices();
        
        while let Some((idx, ch)) = chars.next() {
            if ch == '\'' {
                // Found closing quote
                return Ok((&input[idx..], result));
            } else if ch == '\\' {
                // Check next character
                if let Some((_, next_ch)) = chars.next() {
                    match next_ch {
                        '\'' => result.push('\''),  // Escaped single quote
                        '\\' => result.push('\\'),  // Escaped backslash
                        _ => {
                            // In single quotes, other escapes are literal
                            result.push('\\');
                            result.push(next_ch);
                        }
                    }
                } else {
                    result.push('\\');
                }
            } else {
                result.push(ch);
            }
        }
        
        Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Eof)))
    }
}

fn escaped_string(quote: char) -> impl Fn(&str) -> IResult<&str, String> {
    move |input: &str| {
        let mut result = String::new();
        let mut chars = input.char_indices();
        
        while let Some((idx, ch)) = chars.next() {
            if ch == quote {
                // Found closing quote, return the rest of the input after this position
                return Ok((&input[idx..], result));
            } else if ch == '\\' {
                if let Some((_, escaped)) = chars.next() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        '\'' => result.push('\''),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                } else {
                    result.push('\\');
                }
            } else {
                result.push(ch);
            }
        }
        
        Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Eof)))
    }
}

fn operators(input: &str) -> IResult<&str, Token> {
    // Try multi-character operators first
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("->")(input) {
        return Ok((rest, Token::Arrow));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("=>")(input) {
        return Ok((rest, Token::DoubleArrow));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("===")(input) {
        return Ok((rest, Token::Identical));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("!==")(input) {
        return Ok((rest, Token::NotIdentical));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("==")(input) {
        return Ok((rest, Token::EqualEqual));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("!=")(input) {
        return Ok((rest, Token::NotEqual));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("<=")(input) {
        return Ok((rest, Token::LessThanEqual));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>(">=")(input) {
        return Ok((rest, Token::GreaterThanEqual));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("&&")(input) {
        return Ok((rest, Token::And));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("||")(input) {
        return Ok((rest, Token::Or));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("++")(input) {
        return Ok((rest, Token::PlusPlus));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("--")(input) {
        return Ok((rest, Token::MinusMinus));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("+=")(input) {
        return Ok((rest, Token::PlusEqual));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("-=")(input) {
        return Ok((rest, Token::MinusEqual));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("*=")(input) {
        return Ok((rest, Token::StarEqual));
    }
    if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>("/=")(input) {
        return Ok((rest, Token::SlashEqual));
    }

    // Single-character operators
    alt((
        value(Token::Plus, char('+')),
        value(Token::Minus, char('-')),
        value(Token::Star, char('*')),
        value(Token::Slash, char('/')),
        value(Token::Percent, char('%')),
        value(Token::Equal, char('=')),
        value(Token::LessThan, char('<')),
        value(Token::GreaterThan, char('>')),
        value(Token::Not, char('!')),
        value(Token::Dot, char('.')),
        value(Token::Question, char('?')),
        value(Token::Colon, char(':')),
    ))(input)
}

fn delimiters(input: &str) -> IResult<&str, Token> {
    alt((
        value(Token::LeftParen, char('(')),
        value(Token::RightParen, char(')')),
        value(Token::LeftBrace, char('{')),
        value(Token::RightBrace, char('}')),
        value(Token::LeftBracket, char('[')),
        value(Token::RightBracket, char(']')),
        value(Token::Semicolon, char(';')),
        value(Token::Comma, char(',')),
    ))(input)
}