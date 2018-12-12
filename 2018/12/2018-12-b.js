const input = require("./input");

const TARGET_GENERATION = 50000000000;

const pots = new Map(input.initial.split("").entries());
const rules= new Map(input.rules.split("\n").map(line => {
    let match = line.match(/^([.#]{5}) => ([.#])$/);
    return [ match[1], match[2] ];
}));

function iterate() {
    const readCopy = new Map(pots);
    const ids = Array.from(pots.keys());
    const leftmostPot = ids.reduce((prev, id) => Math.min(prev, id)) - 2;
    const rightmostPot = ids.reduce((prev, id) => Math.max(prev, id)) + 2;
    for (let potId = leftmostPot; potId <= rightmostPot; potId++) {
        const vicinity = [-2, -1, 0, +1, +2].map(offset => readCopy.has(potId + offset) ? readCopy.get(potId + offset) : ".").join("");
        if (rules.has(vicinity) && rules.get(vicinity) === "#") {
            pots.set(potId, "#");
        } else {
            pots.delete(potId);
        }
    }
}

let lastSum = null;
let lastDiff = null;
for (let generation = 1; generation <= 1000; generation++) {
    iterate();
    const sum = Array.from(pots.keys()).reduce((sum, key) => sum + key, 0);
    const diff = sum - lastSum;
    if (lastDiff === diff) {
        console.log(`difference stayed constant at ${diff} after generation ${generation - 1}`);
        const result = sum + diff * (TARGET_GENERATION - generation);
        console.log(`the current generation, ${generation}, has a sum of ${sum}`);
        console.log(`in generation ${TARGET_GENERATION} the sum should be ${result}`);
        break;
    }
    lastSum = sum;
    lastDiff = diff;
}
