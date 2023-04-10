<?php
// This file is used to gzcompress text.json into gzipped-text.json

$sourceFile = 'text.json';
$destinationFile = 'base64-encoded.json';

$json = file_get_contents($sourceFile);
$base64_encoded = base64_encode($json);
file_put_contents($destinationFile, $base64_encoded);
