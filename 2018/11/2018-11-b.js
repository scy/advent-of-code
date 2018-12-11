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
for (let size = 1; size <= 300; size++) {
    console.log(size);
    for (let topY = 1; topY <= HEIGHT - size + 1; topY++) {
        for (let topX = 1; topX <= HEIGHT - size + 1; topX++) {
            let power = 0;
            for (let yOffset = 0; yOffset < size; yOffset++) {
                for (let xOffset = 0; xOffset < size; xOffset++) {
                    power += grid[topY + yOffset - 1][topX + xOffset - 1];
                }
            }
            if (power > best.power) {
                best = {x: topX, y: topY, size: size, power};
            }
        }
    }
}

console.log(best);
