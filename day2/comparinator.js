function test(a, b) {

  let differences = 0;
  for(let i in a) {
    if (a[i] != b[i]) {
      differences++;
      if (differences > 1) {
        return false;
      }
    }
  }

  return differences === 1;
}

function findPairs(inputList) {
  const pairs = [];
  for(let i = 0; i < inputList.length; i++) {
    const iValue = inputList[i];
    if (i === inputList.length - 1)
    {
      continue;
    }
    for(let j = i + 1; j < inputList.length; j++) {
      const jValue = inputList[j];
      if (test(iValue, jValue)) {
        pairs.push([iValue, jValue]);
      }
    }
  }

  return pairs;
}

function findUniqueLetters(a, b) {
  let common = '';
  for(let i in a) {
    if (a[i] == b[i]) {
      common += a[i];
    }
  }
  return common;
}

function process(inputList) {
  const pairs = findPairs(inputList);
  if (pairs.length > 1) {
    console.log(paris);
    throw new Error("Foudn too many pairs");
  }
  const uniqueLetters = findUniqueLetters(pairs[0][0], pairs[0][1]);
  return uniqueLetters;
}

module.exports = {
  test:test,
  findPairs: findPairs,
  process:process
}