"use strict";

const input = require('./input').input;
const alphabet = Array.from('abcdefghijklmopqrstuvwxyz');

function react(polymer) {
    let max_i = polymer.length - 1;
    for (let i = 0; i < max_i; i++) {
        if (polymer[i].toLowerCase() === polymer[i+1].toLowerCase() && polymer[i] !== polymer[i+1]) {
            polymer = polymer.slice(0, i) + polymer.slice(i + 2);
            i = Math.max(-1, i - 2);
            max_i -= 2;
        }
    }
    return polymer;
}

console.log(alphabet.reduce((prev, letter) => {
    let length = react(input.replace(new RegExp(letter, "gi"), "")).length;
    return length < prev.length ? {letter, length} : prev
},  {length: Infinity}));
