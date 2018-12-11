"use strict";

const GRID_SERIAL_NO = 3999;
const WIDTH = 300;
const HEIGHT = 300;

const grid = [];
const integral = [];

function squareSum([x1, y1], [x2, y2]) { // x/y are 1-based, but the integral array is 0-based
    // console.log([x1, y1], [x2, y2]);
    let sum = integral[y2 - 1][x2 - 1];
    if (y1 > 1) { // there are lines above me
        sum -= integral[y1 - 2][x2 - 1];
    }
    if (x1 > 1) { // there are lines left of me
        sum -= integral[y2 - 1][x1 - 2];
    }
    if (x1 > 1 && y1 > 1) {
        sum += integral[y1 - 2][x1 - 2];
    }
    return sum;
}

for (let y = 1; y <= HEIGHT; y++) {
    const line = [];
    grid.push(line);
    const integralLine = [];
    integral.push(integralLine);

    for (let x = 1; x <= WIDTH; x++) {
        const rackID = x + 10;
        let powerLevel = String((rackID * y + GRID_SERIAL_NO) * rackID);
        powerLevel = Number((powerLevel.length < 3) ? 0 : powerLevel[powerLevel.length - 3]) - 5;
        line.push(powerLevel);

        integralLine.push(
            (x > 1 ? integral[y - 1][x - 2] : 0) +
            (y > 1 ? integral[y - 2][x - 1] : 0) -
            (x > 1 && y > 1 ? integral[y - 2][x - 2] : 0) +
            powerLevel
        );
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
