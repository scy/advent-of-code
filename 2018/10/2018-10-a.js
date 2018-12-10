const input = require("./input.js").input;

/* Example input
const input = `position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>`;
*/

const points = input.split("\n").map(line => {
    let [, posX, posY, velX, velY] = line.match(/^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$/);
    return { posX: +posX, posY: +posY, velX: +velX, velY: +velY };
});

function boundingBox(points) {
    return points.reduce((prev, point) => {
        let result = {
            minX: Math.min(point.posX, prev.minX),
            minY: Math.min(point.posY, prev.minY),
            maxX: Math.max(point.posX, prev.maxX),
            maxY: Math.max(point.posY, prev.maxY),
        };
        result.width = result.maxX - result.minX;
        result.height = result.maxY - result.minY;
        result.area = result.width * result.height;
        return result;
    }, { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity });
}

function dump(points, bb) {
    bb = bb || boundingBox(points);
    const bitmap = [];
    for (let y = bb.minY; y <= bb.maxY; y++) {
        bitmap.push(new Array(bb.maxX - bb.minX + 1).fill("."));
    }
    for (let point of points) {
        bitmap[point.posY - bb.minY][point.posX - bb.minX] = "#";
    }
    for (let line of bitmap) {
        console.log(line.join(""));
    }
}

function iterate(points, factor = 1) {
    for (let point of points) {
        point.posX += factor * point.velX;
        point.posY += factor * point.velY;
    }
}

let prevArea = Infinity;
for (let i = 0; i < 100000; i++) {
    const bb = boundingBox(points);
    if (bb.area > prevArea) {
        iterate(points, -1);
        dump(points, bb);
        break;
    }
    prevArea = bb.area;
    iterate(points);
}
