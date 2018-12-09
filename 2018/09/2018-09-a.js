"use strict";

// Puzzle input.
const PLAYERS = 479;
const MAX_MARBLE = 71035;

const circle = [ 0 ];
const score = {};
let current_player = 0;
let marble_index = 0;

for (let current_value = 1; current_value <= MAX_MARBLE; current_value++) {
    if (current_value % 23 === 0) {
        // Create score entry for player, if not existing.
        if (!(current_player in score)) {
            score[current_player] = 0;
        }

        let to_remove = marble_index - 7;
        if (to_remove < 0) {
            to_remove = circle.length + to_remove;
        }
        const [ removed ] = circle.splice(to_remove, 1);
        marble_index = to_remove % circle.length;

        score[current_player] += current_value + removed;
    } else {
        const insert_before = (marble_index + 2) % circle.length;
        if (insert_before === 0) {
            circle.push(current_value);
            marble_index = circle.length - 1;
        } else {
            circle.splice(insert_before, 0, current_value);
            marble_index = insert_before;
        }
    }

    // Next player.
    current_player = (current_player + 1) % PLAYERS;
}

const winner = Object.entries(score).reduce((prev, [ player, score ]) => score > prev.score ? {player: +player + 1, score} : prev, {score: -Infinity});

console.log(winner);
