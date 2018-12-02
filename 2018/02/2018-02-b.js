"use strict";

let readline = require("readline").createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false, // else the input will be echoed
});

let lines = [];
readline.on("line", line => lines.push(line));
readline.on("close", () => {
    let number = lines.length - 1;
    for (let i = 0; i < number; i++) {
        let line = lines[i];
        for (let j = i + 1; j < lines.length; j++) {
            let other_line = lines[j];
            let samechars = "";
            for (let pos = 0; pos < line.length; pos++) {
                if (line[pos] === other_line[pos]) {
                    samechars += line[pos];
                }
            }
            if (samechars.length === line.length - 1) {
                console.log(samechars);
            }
        }
    }
});
