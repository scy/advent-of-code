"use strict";

// Puzzle input.
const PLAYERS = 479;
const MAX_MARBLE = 71035 * 100;

function counterclockwise(marble, steps) {
    for (let i = 0; i < steps; i++) {
        marble = marble.prev;
    }
    return marble;
}

function insertAfter(after, to_insert) {
    to_insert.next = after.next;
    after.next = to_insert;
    to_insert.prev = after;
    to_insert.next.prev = to_insert;
}

function remove(to_remove) {
    to_remove.prev.next = to_remove.next;
    to_remove.next.prev = to_remove.prev;
    to_remove.prev = to_remove.next = null;
    return to_remove;
}

function listToArray(start_marble) {
    const array = [ start_marble.value ];
    for (let current = start_marble.next; current !== start_marble; current = current.next) {
        array.push(current.value);
    }
    return array;
}

const score = {};
let current_player = 0;

let zero = { value: 0 };
zero.next = zero.prev = zero;
let current_marble = zero;

for (let current_value = 1; current_value <= MAX_MARBLE; current_value++) {
    if (current_value % 23 === 0) {
        if (!(current_player in score)) {
            score[current_player] = 0;
        }
        const toRemove = counterclockwise(current_marble, 7);
        const future_current_marble = toRemove.next;
        const removed = remove(toRemove);
        current_marble = future_current_marble;
        score[current_player] += current_value + removed.value;
    } else {
        const new_marble = {value: current_value};
        insertAfter(current_marble.next, new_marble);
        current_marble = new_marble;
    }

    current_player = (current_player + 1) % PLAYERS;
}

const winner = Object.entries(score).reduce((prev, [ player, score ]) => score > prev.score ? {player: +player + 1, score} : prev, {score: -Infinity});

console.log(winner);
