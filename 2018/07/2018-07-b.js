const input = require("./input").input;

/* const input = `Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.`.split("\n").map(line => {
    const match = line.match(/^Step (.) must be finished before step (.) can begin.$/);
    return { first: match[1], then: match[2] };
}); */


const prev = {};
const letters = new Set();
const timeLeftFor = {};
const built = [];
const workers = 5;
const delay = 60;

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

function findCandidates() {
    return Array.from(letters.values()).filter(letter => ready(letter) && !built.includes(letter) && !(letter in timeLeftFor)).sort();
}

function startWorkingOn(letter) {
    timeLeftFor[letter] = letter.charCodeAt(0) - 64 + delay;
}

function inProgressCount() {
    return Object.keys(timeLeftFor).length;
}

function updateWorkStatus() {
    for (letter of Object.keys(timeLeftFor)) {
        if (--timeLeftFor[letter] === 0) {
            built.push(letter);
            delete(timeLeftFor[letter]);
        }
    }
}

prepareData();

let seconds = 0;
while (true) {
    updateWorkStatus();
    const candidates = findCandidates();
    if (candidates.length) {
        while (candidates.length && inProgressCount() < workers) {
            startWorkingOn(candidates.shift());
        }
    } else if (inProgressCount() === 0) {
        break;
    }
    console.log(timeLeftFor);
    seconds++;
}

console.log(seconds);

// console.log(built.join(""));
