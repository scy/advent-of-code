"use strict";

const readline = require("readline").createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false, // else the input will be echoed
});

const log = [];
readline.on("line", line => {
    const [, year, month, day, hour, minute, text] = line.match(/^\[(....)-(..)-(..) (..):(..)\] (.+)$/);
    const date = new Date(Date.UTC(+year, +month - 1, +day, +hour, +minute));
    log.push({date, text});
});
readline.on("close", () => {
    log.sort((a, b) => a.date - b.date);
    const sleeptimes = {};
    const sleeptotals = {};
    let guard;
    let asleep_since;
    for (const entry of log) {
        let match;
        if (match = entry.text.match(/^Guard #(\d+) begins shift$/)) {
            guard = +match[1];
        } else if (entry.text === "falls asleep") {
            asleep_since = entry.date.getUTCMinutes();
        } else if (entry.text === "wakes up") {
            if (!(guard in sleeptimes)) {
                sleeptimes[guard] = new Array(60).fill(0, 0, 60);
                sleeptotals[guard] = 0;
            }
            const woke_up_at = entry.date.getUTCMinutes();
            sleeptotals[guard] += woke_up_at - asleep_since;
            for (let minute = asleep_since; minute < woke_up_at; minute++) {
                sleeptimes[guard][String(minute)] += 1;
            }
        } else {
            console.log("WTF");
        }
    }
    let max_sleep = 0;
    let best_guard;
    for (let [guard, total] of Object.entries(sleeptotals)) {
        if (total > max_sleep) {
            max_sleep = total;
            best_guard = guard;
        }
    }
    let best_minute = sleeptimes[best_guard].reduce((prev, count, minute) => count > prev.count ? {minute, count} : prev, {minute: false, count: 0});
    console.log(`${best_guard} is the best guard, slept ${max_sleep} minutes, the most in minute ${best_minute.minute}`, sleeptimes[best_guard]);
    console.log(`result is ${best_guard * best_minute.minute}`);
});
