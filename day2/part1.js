const checksummer = require('./checksummer');
const fs = require('fs');

const inputString = fs.readFileSync('part1Input.txt', {encoding:"utf8"});

const inputArray = inputString.split("\n");

const result = checksummer.process(inputArray);

console.log(result);