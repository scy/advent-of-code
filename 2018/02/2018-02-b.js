"use strict";

let readline = require("readline").createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false, // else the input will be echoed
});

let lines = [];
readline.on("line", line => lines.push(line));
readline.on("close", () => {
    outer_lines: for (let line of lines) {
        for (let other_line of lines) {
            let samechars = "";
            for (let pos = 0; pos < line.length; pos++) {
                if (line[pos] === other_line[pos]) {
                    samechars += line[pos];
                }
            }
            if (samechars.length === line.length - 1) {
                console.log(samechars);
                break outer_lines;
            }
        }
    }
});
