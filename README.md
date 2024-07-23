# PHP Tools

> [!NOTE]
> Language References:
> - [PHP Language Reference](https://www.php.net/manual/en/langref.php)
> - [PHP Language Specification](https://phplang.org/welcome.html)
> - [Zend Grammar Definition](https://github.com/php/php-src/blob/master/Zend/zend_language_parser.y)
> - [PHP Parser Grammar Definition](https://github.com/nikic/PHP-Parser/blob/master/grammar/php.y)
> 
> Addiotional Resources:
> - [PHP Internals Book](https://www.phpinternalsbook.com/)
> - [PHP Watch](https://php.watch/)
> - [Crafting Interpreters](https://craftinginterpreters.com/contents.html)

## Introduction

Experimental project to improve the tooling for PHP development.

## Status

This project is in the very very very early stages of development, currently building the parser.

Main tech used:

- [`pest`](https://crates.io/crates/pest) for grammar definition 
- [`rowan`](https://crates.io/crates/rowan) for AST generation

## Goals:

Inspired by Rust's ecosystem tools such as `rustc` and `rust-analyze`:

- Parse
- Lint
- Format
- Improved Errors and Suggestions

Inspired by WP Playground and the potential use of PHP on the web:

- Minimize
- Bundle

Inspired by *Wouldn't it be cool?*:

- Interpreter and runtime