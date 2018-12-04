const repeatFinder = require('./repeatFinder');
const fs = require('fs');

const inputText = fs.readFileSync("./input.txt", {encoding:"utf8"});

console.log(repeatFinder(inputText));