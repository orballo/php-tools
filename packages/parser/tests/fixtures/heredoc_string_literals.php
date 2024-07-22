<?php

// Basic heredoc string with double quotes
$basic = <<<"EOD"
This is a basic heredoc string.
It can span multiple lines.
EOD;

// Basic heredoc string without double quotes
$basic_without_quotes = <<<EOD
This is a basic heredoc string.
It can span multiple lines.
EOD;

// Heredoc with interpolated variables
$name = "Alice";
$vars = <<<EOD
Hello, $name!
Variables are not interpolated in heredoc strings.
EOD;

// Heredoc with quotes and special characters
$special = <<<EOD
Heredoc strings can contain "double quotes" and 'single quotes'.
Special characters like \n \t are treated literally.
EOD;

// Heredoc with HTML
$html = <<<HTML_CODE
<div class="example">
    <p>This is an HTML snippet in a heredoc string.</p>
</div>
HTML_CODE;

// Heredoc with indentation (preserves whitespace)
$indented = <<<PHP_CODE
    function example() {
        echo "This code block preserves indentation";
    }
PHP_CODE;

// Empty heredoc string
$empty = <<<EMPTY
EMPTY;

// Heredoc containing PHP code
$php_code = <<<PHP_SCRIPT
<?php
$array = [1, 2, 3];
foreach ($array as $value) {
    echo $value;
}
?>
PHP_SCRIPT;

?>