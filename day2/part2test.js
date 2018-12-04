const comparinator = require('./comparinator');
const assert = require('assert');

const inputIds = [
  "abcde",
  "fghij",
  "klmno",
  "pqrst",
  "fguij",
  "axcye",
  "wvxyz"
];

assert(comparinator.test("abcde", "abcdf"), "test positive");
assert(comparinator.test("egggg", "eaggg"), "test positive");
assert(!comparinator.test("abcde", "fhijk"), "test negative");

const pairs = comparinator.findPairs(inputIds);
assert(pairs.length === 1, "find one pair");

const common = comparinator.process(inputIds);

assert(common === "fgij", "process");

console.log("win!");
