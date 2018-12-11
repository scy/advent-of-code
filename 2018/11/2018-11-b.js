"use strict";

const GRID_SERIAL_NO = 3999;
const WIDTH = 300;
const HEIGHT = 300;

const grid = [];
const integral = [ Array(WIDTH).fill(0) ];

function squareSum([x1, y1], [x2, y2]) { // all arrays are 1-based
    // console.log([x1, y1], [x2, y2]);
    return integral[y2][x2] - integral[y1 - 1][x2] - integral[y2][x1 - 1] + integral[y1 - 1][x1 - 1];
}

for (let y = 1; y <= HEIGHT; y++) {
    const line = [];
    grid.push(line);
    const integralLine = [ 0 ];
    integral.push(integralLine);

    for (let x = 1; x <= WIDTH; x++) {
        const rackID = x + 10;
        let powerLevel = String((rackID * y + GRID_SERIAL_NO) * rackID);
        powerLevel = Number((powerLevel.length < 3) ? 0 : powerLevel[powerLevel.length - 3]) - 5;
        line.push(powerLevel);

        integralLine.push(integral[y][x - 1] + integral[y - 1][x] - integral[y - 1][x - 1] + powerLevel);
    }
}

let best = {power: -Infinity};
for (let size = 1; size <= 300; size++) {
    for (let topY = 1; topY <= HEIGHT - size; topY++) {
        for (let topX = 1; topX <= HEIGHT - size; topX++) {
            let power = squareSum([topX, topY], [topX + size - 1, topY + size - 1]);
            if (power > best.power) {
                best = {x: topX, y: topY, size: size, power};
            }
        }
    }
}

console.log(best);
