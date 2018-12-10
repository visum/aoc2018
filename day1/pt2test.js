const repeatFinder = require('./repeatFinder');
const fs = require('fs');

const case1 = {
  inputFile:"pt2Test1.txt",
  result: 0
};

const case2 = {
  inputFile:"pt2Test2.txt",
  result:10
};

const case3 = {
  inputFile:"pt2Test3.txt",
  result:5
};

const case4 = {
  inputFile:"pt2Test4.txt",
  result:14
};

function runCase(testCase) {
  const inputText = fs.readFileSync(testCase.inputFile, {encoding:"utf8"});

  const result = repeatFinder(inputText);
  if (result !== testCase.result) {
    throw new Error(`Expected ${testCase.result}, but got ${result}`);
  }
}

runCase(case1);
runCase(case2);
runCase(case3);
runCase(case4);

console.log("Win!");