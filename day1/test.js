const decoder = require('./decoder');
const fs = require('fs');

const case1 = {
  inputPath: "testInput1.txt",
  result: 3
};

const case2 = {
  inputPath: "testInput2.txt",
  result: 0
};

const case3 = {
  inputPath: "testInput3.txt",
  result: -6
}

function runCase(testCase) {
  const inputText = fs.readFileSync(testCase.inputPath, {encoding: "utf8"});
  const result = decoder(inputText);
  if (result !== testCase.result) {
    throw new Error("Test case failed with input " + testCase.inputPath + ". Expected " + testCase.result + " but got " + result);
  }
}

runCase(case1);
runCase(case2);
runCase(case3);

console.log("win!");