"use strict";

const readline = require("readline").createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false, // else the input will be echoed
});

const fabric = [];
const no_overlap = new Set();
let max_x = 0;
let max_y = 0;
readline.on("line", line => {
    const match = line.match(/^#(?<id>\d+) @ (?<x>\d+),(?<y>\d+): (?<w>\d+)x(?<h>\d+)$/);
    no_overlap.add(+match.groups.id);
    for (let y = +match.groups.y; y < +match.groups.y + +match.groups.h; y++) {
        max_y = Math.max(max_y, y);
        if (!(y in fabric)) {
            fabric[y] = [];
        }
        for (let x = +match.groups.x; x < +match.groups.x + +match.groups.w; x++) {
            if (x in fabric[y]) {
                fabric[y][x].push(+match.groups.id);
            } else {
                fabric[y][x] = [+match.groups.id];
            }
            max_x = Math.max(max_x, x);
        }
    }
});
readline.on("close", () => {
    for (let y = 0; y <= max_y; y++) {
        if (y in fabric) {
            for (let x = 0; x <= max_x; x++) {
                if (x in fabric[y]) {
                    if (fabric[y][x].length > 1) {
                        for (let claim of fabric[y][x]) {
                            no_overlap.delete(claim);
                        }
                    }
                }
            }
        }
    }
    console.log(no_overlap);
});
