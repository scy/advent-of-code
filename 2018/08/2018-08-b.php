<?php

$input = file('input.js');
// $input = ['2 3 0 3 10 11', '12 1 1 0 1 99 2 1 1 2'];

$input = array_reduce($input, function ($carry, $line) {
    if (preg_match('/^[0-9 ]+$/', $line)) {
        foreach (explode(' ', trim($line)) as $number) {
            $carry[] = (int)$number;
        }
    }
    return $carry;
});

class Node {
    protected $numChildNodes;
    protected $numMetadataEntries;
    protected $childNodes;
    protected $metadataEntries;

    function __construct(&$input)
    {
        $this->numChildNodes = array_shift($input);
        $this->numMetadataEntries = array_shift($input);
        $this->childNodes = [];
        $this->metadataEntries = [];

        for ($i = 0; $i < $this->numChildNodes; $i++) {
            $this->childNodes[] = new Node($input);
        }

        for ($i = 0; $i < $this->numMetadataEntries; $i++) {
            $this->metadataEntries[] = array_shift($input);
        }
    }

    function getMetadataSum() {
        if (!$this->childNodes) {
            return array_sum($this->metadataEntries);
        }
        return array_reduce($this->metadataEntries, function ($prev, $reference) {
            $reference--;
            return $prev +
                (array_key_exists($reference, $this->childNodes) ? $this->childNodes[$reference]->getMetadataSum() : 0);
        }, 0);
    }
}

$root = new Node($input);
var_dump($root->getMetadataSum());
