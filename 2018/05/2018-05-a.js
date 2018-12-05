"use strict";

const input = require('./input').input;
let result = input;
let max_i = input.length - 1;


for (let i = 0; i < max_i; i++) {
    if (result[i].toLowerCase() === result[i+1].toLowerCase() && result[i] !== result[i+1]) {
        result = result.slice(0, i) + result.slice(i + 2);
        i = Math.max(-1, i - 2);
        max_i -= 2;
    }
}

console.log(result.length);
