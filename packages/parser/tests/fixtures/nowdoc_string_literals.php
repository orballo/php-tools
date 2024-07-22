<?php

// Basic nowdoc string
<<<'EOD'
This is a basic nowdoc string.
It can span multiple lines.
EOD;

// Nowdoc with variables (not interpolated)
<<<'EOD'
Hello, $name!
Variables are not interpolated in nowdoc strings.
EOD;

// Nowdoc with quotes and special characters
<<<'EOD'
Nowdoc strings can contain "double quotes" and 'single quotes'.
Special characters like \n \t are treated literally.
EOD;

// Nowdoc with HTML
<<<'HTML_CODE'
<div class="example">
    <p>This is an HTML snippet in a nowdoc string.</p>
</div>
HTML_CODE;

// Nowdoc with indentation (preserves whitespace)
<<<'PHP_CODE'
    function example() {
        echo "This code block preserves indentation";
    }
PHP_CODE;

// Nowdoc with an empty string
<<<'EMPTY'
EMPTY;

// Nowdoc containing PHP code
<<<'PHP_SCRIPT'
<?php
$array = [1, 2, 3];
foreach ($array as $value) {
    echo $value;
}
?>
PHP_SCRIPT;

?>