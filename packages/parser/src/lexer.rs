use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum Token<'source> {
    #[token("<?")]
    OpenTag,

    #[token("?>")]
    CloseTag,

    #[regex(r"\n|\r\n")]
    Newline,

    #[regex(r"[ \t\f]+", logos::skip)]
    Whitespace,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Assign,

    #[regex(r"[+-]?(:?0|[1-9][0-9]*)")]
    Integer(&'source str),

    #[regex(r"'(?:[^'\\]|\\.)*'", |lex| lex.slice().trim_matches('\''))]
    String(&'source str),

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("null")]
    Null,

    #[token("echo")]
    Echo,

    #[token(r"$[_a-zA-Z][_a-zA-Z0-9]*")]
    Variable(&'source str),

    #[token(r"[_a-zA-Z][_a-zA-Z0-9]*")]
    Identifier(&'source str),
}
