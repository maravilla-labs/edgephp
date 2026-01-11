# Changelog

All notable changes to EdgePHP will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions workflow for automated playground deployment
- Apache 2.0 license
- CONTRIBUTING.md with contribution guidelines
- ROADMAP.md with feature status and plans

### Changed
- Reorganized documentation for open-source release
- Updated project metadata in Cargo.toml

## [0.1.0] - 2025-01-11

### Added
- PHP-to-WebAssembly compiler
- Parser for PHP syntax using nom combinators
- Support for variables, arithmetic, and string operations
- All comparison and logical operators
- Type casting and PHP-compliant type coercion
- Control flow: if/else, while, do-while, for, foreach, switch
- Break and continue statements
- Ternary operator
- User-defined functions with parameters and return values
- Arrays (indexed and associative) with 9 built-in functions
- Basic OOP: classes, properties, methods, constructors
- 30+ built-in functions across strings, arrays, math, and type checking
- Reference counting garbage collection with cycle detection
- Loop unrolling and escape analysis optimizations
- Command-line interface for parsing and compilation
- React-based web playground with Monaco editor
- Performance metrics display in playground

### Performance
- Sub-millisecond execution for simple operations
- Efficient 16-byte PhpValue representation
- Sub-second compilation for typical programs
