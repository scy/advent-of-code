// const input = `2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2`.trim().split(/\s+/).map(number => +number);

const input = require("./input").input;

class Node {
    constructor(input) {
        this._numChildNodes = input.shift();
        this._numMetadataEntries = input.shift();
        this._childNodes = [];
        this._metadataEntries = [];

        for (let i = 0; i < this._numChildNodes; i++) {
            this._childNodes.push(new Node(input));
        }

        for (let i = 0; i < this._numMetadataEntries; i++) {
            this._metadataEntries.push(input.shift());
        }
    }

    get metadataSum() {
        return this._childNodes.reduce((prev, node) => prev + node.metadataSum, 0) +
            this._metadataEntries.reduce((prev, entry) => prev + entry, 0);
    }
}

const root = new Node(input);
console.log(root.metadataSum);
