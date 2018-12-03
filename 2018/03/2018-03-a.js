"use strict";

const readline = require("readline").createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false, // else the input will be echoed
});

const fabric = [];
let max_x = 0;
let max_y = 0;
let multiclaims = 0;
readline.on("line", line => {
    const match = line.match(/^#(?<id>\d+) @ (?<x>\d+),(?<y>\d+): (?<w>\d+)x(?<h>\d+)$/);
    for (let y = +match.groups.y; y < +match.groups.y + +match.groups.h; y++) {
        max_y = Math.max(max_y, y);
        if (!(y in fabric)) {
            fabric[y] = [];
        }
        for (let x = +match.groups.x; x < +match.groups.x + +match.groups.w; x++) {
            if (x in fabric[y]) {
                if (fabric[y][x]++ === 1) {
                    multiclaims++;
                }
            } else {
                fabric[y][x] = 1;
            }
            max_x = Math.max(max_x, x);
        }
    }
});
readline.on("close", () => {
    console.log(max_x, max_y, multiclaims);
});
