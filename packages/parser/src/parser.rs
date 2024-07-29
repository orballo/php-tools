// @TODO: the parser core implementation should parse
// the pieces of the scripts, and once the script sections
// are identified, these are the ones that should be parsed
// with the extensions
//
// The text sections could potentially accept extensions to parse HTML, etc.

pub struct Parser {}

impl Parser {
    pub fn builder() -> ParserBuilder {
        ParserBuilder::new()
    }
}

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
//
pub struct ParserBuilder {
    tokens: Vec<Token>,
    parsers: Vec<Box<dyn SubParser>>,
}

impl ParserBuilder {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            parsers: Vec::new(),
        }
    }

    pub fn parser() -> Self {
        todo!()
    }

    pub fn parsers() -> Self {
        todo!()
    }
}

pub trait SubParser {}
