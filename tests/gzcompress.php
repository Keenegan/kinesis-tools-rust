<?php

$json = file_get_contents("input");
$compressed = gzcompress(trim($json));
file_put_contents("output-gzcompressed", $compressed);
