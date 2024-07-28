use lexer::lex;

fn main() {
    let php = "<?php echo \"Hello World\"; ?>";
    let tokens = lex(php);
}
