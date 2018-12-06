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

function distanceToCoordinates([x, y], input) {
    return input.reduce((prev, coordinate, idx) => {
        return prev + distanceBetween([x, y], coordinate);
    }, 0);
}

function populate(min_x, max_x, min_y, max_y, input) {
    const field = [];
    for (let x = min_x; x <= max_x; x++) {
        field[x] = [];
        for (let y = min_y; y <= max_y; y++) {
            field[x][y] = distanceToCoordinates([x, y], input);
        }
    }
    return field;
}

function closeRegionSize(field) {
    return field.reduce((prev, column) => {
        return prev + column.filter(sum => sum < 10000).length;
    }, 0);
}

let field = populate(min_x, max_x, min_y, max_y, input);
console.log(closeRegionSize(field));
