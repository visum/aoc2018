function findRepeats(input) {
  const result = {};
  const checkedLetters = [];
  for (const char of input) {
    if (checkedLetters.includes(char)) {
      continue;
    }
    const regexp = new RegExp('(' + char + ')', "g");
    const matches = input.match(regexp).length;

    const previousMatches = result[matches] || 0;
    result[matches] = previousMatches + 1;
    checkedLetters.push(char);
  }

  return result;
}

function process(idsList) {
  const counts = idsList.reduce((previous, current) => {
    const repeats = findRepeats(current);
    repeats[2] && previous[2]++;
    repeats[3] && previous[3]++;
    return previous;
  }, {2:0,3:0});

  return counts[2] * counts[3];
};

module.exports = {
  process: process,
  findRepeats: findRepeats
};