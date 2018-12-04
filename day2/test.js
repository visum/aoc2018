const checksummer = require('./checksummer');
const assert = require('assert');

const boxIds = [
  "abcdef",
  "bababc",
  "abbcde",
  "abcccd",
  "aabcdd",
  "abcdee",
  "ababab"
];


const matches = boxIds.map(i => checksummer.findRepeats(i));

console.log(matches);

assert(!matches[0][2], "no matches in 0");
assert(matches[1][2] === 1, "1 gropu of 2 in 1");
assert(matches[1][3] === 1, "1 group of 3 in 1");
assert(matches[2][2] === 1, "1 group of 2 in 2");
assert(!matches[2][3], "no group of 3 in 2");


const expectedResult = 12;
const result = checksummer.process(boxIds);

if (result !== expectedResult) {
  throw new Error(`Got ${result}, expected ${expectedResult}`);
}

console.log("Win!");