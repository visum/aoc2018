module.exports = function(inputText) {
  const inputArray = inputText.split("\n").map(i => parseFloat(i));
  const results = [0];
  let match = null;
  let i = 0;
  let accumulator = 0;

  while(match === null) {
    const current = inputArray[i];
    accumulator += current;

    if (results.indexOf(accumulator) > -1) {
      match = accumulator;
    }

    results.push(accumulator);
    
    i++;
    if (i >= inputArray.length) {
      i=0;
    }
  }

  return match;

}