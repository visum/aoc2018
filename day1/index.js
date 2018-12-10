const decoder = require('./decoder');
const fs = require('fs');

const inputString = fs.readFileSync("input.txt", {encoding: "utf8"});

console.log(decoder(inputString));
