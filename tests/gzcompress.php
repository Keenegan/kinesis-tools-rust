<?php
// This file is used to gzcompress text.json into gzipped-text.json

$sourceFile = 'text.json';
$destinationFile = 'gzipped-text.json';

$json = file_get_contents("$sourceFile");
$jsonEncoded = json_encode($json);
$compressed = gzcompress($jsonEncoded);
file_put_contents($destinationFile, $compressed);
