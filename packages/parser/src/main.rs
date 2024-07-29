use lexer::PHPLexer;
use parser::parser::{PHPParser, SyntaxKind};

fn main() {
    let php = "<?php echo \"Hello World\"; ?>";
    let lexer = PHPLexer::new();
    let mut parser = PHPParser::new(php, lexer);

    parser.parse();

    let result = parser.finish();

    let root: SyntaxKind = result.kind().into();

    dbg!(root);
}
