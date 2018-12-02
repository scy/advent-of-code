"use strict";

const readline = require("readline").createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false, // else the input will be echoed
});

const lines = [];
readline.on("line", line => lines.push(line));
readline.on("close", () => {
    const lettercounts = new Map();
    let doubles = 0;
    let triples = 0;
    for (const line of lines) {
        lettercounts.clear();
        for (const char of line) {
            if (lettercounts.has(char)) {
                lettercounts.set(char, lettercounts.get(char) + 1);
            } else {
                lettercounts.set(char, 1);
            }
        }
        let doubles_found = false;
        let triples_found = false;
        for (const count of lettercounts.values()) {
            if (count === 2) {
                doubles_found = true;
            }
            if (count === 3) {
                triples_found = true;
            }
        }
        doubles += doubles_found ? 1 : 0;
        triples += triples_found ? 1 : 0;
    }
    console.log(doubles, "doubles,", triples, "triples, product is", doubles * triples);
});
