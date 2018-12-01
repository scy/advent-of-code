"use strict";

// reads input values from stdin until stdin is closed

let readline = require("readline").createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false, // else the input will be echoed
});

let lines = [];
readline.on("line", line => lines.push(line));
readline.on("close", () => {
    let current = 0;
    let seen = new Map();
    repeat_list: while (true) {
        for (let line of lines) {
            current += +line;
            if (seen.has(current)) {
                console.log(`${current} was seen before`);
                break repeat_list;
            }
            seen.set(current, true);
        }
    }
});

