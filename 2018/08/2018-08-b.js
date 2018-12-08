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
        if (this._childNodes.length === 0) {
            return this._metadataEntries.reduce((prev, entry) => prev + entry, 0);
        }
        return this._metadataEntries.reduce((prev, reference) => {
            reference--;
            return prev +
                ((reference in this._childNodes) ? this._childNodes[reference].metadataSum : 0);
        }, 0);
    }
}

const root = new Node(input);
console.log(root.metadataSum);
