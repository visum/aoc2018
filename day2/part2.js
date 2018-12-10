const comparinator = require("./comparinator").process;
const fs = require('fs');

const inputString = fs.readFileSync('part1Input.txt', {encoding:"utf8"});

const inputArray = inputString.split("\n");

const result = comparinator(inputArray);

console.log(result);
