"use strict";

let readline = require("readline").createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false, // else the input will be echoed
});

let lines = [];
readline.on("line", line => lines.push(line));
readline.on("close", () => {
    let result = lines.reduce((prev, line) => prev + +line, 0);

    console.log(result);
});
