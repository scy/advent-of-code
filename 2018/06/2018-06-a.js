const input = require("./input").input;

let min_x = input[0][0], min_y = input[0][1], max_x = input[0][0], max_y = input[0][1];
input.forEach(([x, y]) => {
    min_x = Math.min(min_x, x);
    min_y = Math.min(min_y, y);
    max_x = Math.max(max_x, x);
    max_y = Math.max(max_y, y);
});

function distanceBetween([x1, y1], [x2, y2]) {
    return Math.abs(x1 - x2) + Math.abs(y1 - y2);
}

function nearestCoordinate([x, y], input) {
    const closest = input.reduce((prev, coordinate, idx) => {
        const distance = distanceBetween([x, y], coordinate);
        if (distance < prev.min) {
            return {min: distance, closest: idx};
        }
        if (distance === prev.min) {
            return {min: prev.min, closest: null}
        }
        return prev;
    }, {min: Infinity});
    return closest.closest;
}

function populate(min_x, max_x, min_y, max_y, input) {
    const field = [];
    for (let x = min_x; x <= max_x; x++) {
        field[x] = [];
        for (let y = min_y; y <= max_y; y++) {
            field[x][y] = nearestCoordinate([x, y], input);
        }
    }
    return field;
}

function infinites(min_x, max_x, min_y, max_y, field) {
    let infinites = new Set();
    for (let x of [min_x, max_x]) {
        for (let y = min_y; y <= max_y; y++) {
            if (typeof field[x][y] !== "undefined") {
                infinites.add(field[x][y]);
            }
        }
    }
    for (let y of [min_y, max_y]) {
        for (let x = min_x; x <= max_x; x++) {
            if (typeof field[x][y] !== "undefined") {
                infinites.add(field[x][y]);
            }
        }
    }
    return infinites;
}

function largestAreaSize(field, infinites) {
    const area = {};
    for (let column of field) {
        if (typeof column === "undefined") {
            continue;
        }
        for (let spot of column) {
            if (spot !== null && typeof spot !== "undefined") {
                if (!(spot in area)) {
                    area[spot] = 0;
                }
                area[spot]++;
            }
        }
    }
    return Object.entries(area).filter(([coordinate]) => {
        return !infinites.has(+coordinate)
    }).reduce((prev, [coordinate, area,]) => {
        return area > prev.area ? {area, coordinate} : prev
    }, {area: 0});
}

let field = populate(min_x, max_x, min_y, max_y, input);
console.log(largestAreaSize(field, infinites(min_x, max_x, min_y, max_y, field)));
