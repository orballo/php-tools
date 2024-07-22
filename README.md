# PHP Tools

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