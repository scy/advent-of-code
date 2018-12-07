const input = require("./input").input;

const prev = {};
const letters = new Set();
const built = [];

function prepareData() {
    for ({first, then} of input) {
        letters.add(first);
        letters.add(then);
        if (!(then in prev)) {
            prev[then] = [];
        }
        prev[then].push(first);
    }
}

function ready(letter) {
    if (!(letter in prev)) {
        return true;
    }
    for (prereq of prev[letter]) {
        if (!built.includes(prereq)) {
            return false;
        }
    }
    return true;
}

function findNext() {
    const candidates = Array.from(letters.values()).filter(letter => ready(letter) && !built.includes(letter)).sort();
    return candidates ? candidates[0] : false;
}

prepareData();

let nextLetter;
while (nextLetter = findNext()) {
    built.push(nextLetter);
}

console.log(built.join(""));
