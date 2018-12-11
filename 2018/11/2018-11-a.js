"use strict";

const GRID_SERIAL_NO = 3999;
const WIDTH = 300;
const HEIGHT = 300;

const grid = [];

for (let y = 1; y <= HEIGHT; y++) {
    const line = [];
    grid.push(line);
    for (let x = 1; x <= WIDTH; x++) {
        const rackID = x + 10;
        let powerLevel = String((rackID * y + GRID_SERIAL_NO) * rackID);
        powerLevel = Number((powerLevel.length < 3) ? 0 : powerLevel[powerLevel.length - 3]) - 5;
        line.push(powerLevel);
    }
}

let best = {power: -Infinity};
for (let y = 1; y <= HEIGHT - 2; y++) {
    for (let x = 1; x <= HEIGHT - 2; x++) {
        const power = grid[y-1][x-1] + grid[y-1][x] + grid[y-1][x+1] +
            grid[y][x-1] + grid[y][x] + grid[y][x+1] +
            grid[y+1][x-1] + grid[y+1][x] + grid[y+1][x+1];
        if (power > best.power) {
            best = {x, y, power};
        }
    }
}

console.log(best);
