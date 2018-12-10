module.exports = function(inputString) {
  const inputArray = inputString.split("\n");
  return inputArray.map(i => parseFloat(i)).reduce((previous, current) => {
    return previous + current;
  }, 0);
};