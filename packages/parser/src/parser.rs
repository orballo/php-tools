// @TODO: the parser core implementation should parse
// the pieces of the scripts, and once the script sections
// are identified, these are the ones that should be parsed
// with the extensions
//
// The text sections could potentially accept extensions to parse HTML, etc.

// @TODO: We need to implement a Token struct.
// This struct is going to be used to store token kinds
// with their name, span, and &str to source.
//
// We need to do the lexing first, and if possible
// concurrently over multiple files.
//
// Parsing should be done after lexing, and only using
// the tokens from the lexer's output.
//
// Each feature/token of the language will be
// in a package called extension.
//
// The individual extensions can implement:
// - Token definition and parser (Optional)
// - Expresion definition and parser (Optional)
// - Statement definition and parser (Optional)
// - Bytecode instruction definition (Optional)
// - Bytecode instruction parser (Optional)

#[repr(u16)]
pub enum Script {
    Root,
    Text,
    Section,
}

#[repr(u16)]
pub enum SyntaxKind {
    Statement,
    Expression,
    Token,
}

pub struct PHPParser<'source> {
    _builder: rowan::GreenNodeBuilder<'source>,
}

impl<'source> PHPParser<'source> {
    pub fn new() -> PHPParserBuilder<'source> {
        PHPParserBuilder::new()
    }
}

pub struct PHPParserBuilder<'source> {
    _parsers: Vec<Box<dyn Parser<'source, Kind = SyntaxKind>>>,
}

impl<'source> PHPParserBuilder<'source> {
    pub fn new() -> Self {
        Self {
            _parsers: Vec::new(),
        }
    }

    pub fn parser(_parser: Box<dyn Parser<Kind = SyntaxKind>>) -> Self {
        todo!()
    }

    pub fn parsers(_parsers: Vec<Box<dyn Parser<Kind = SyntaxKind>>>) -> Self {
        todo!()
    }

    pub fn build(self) -> PHPParser<'source> {
        todo!()
    }
}

pub trait Parser<'source> {
    type Kind;

    fn parser(&self) -> Box<dyn chumsky::Parser<&'source str, (), Error = chumsky::error::Cheap<&str>>>;
}

