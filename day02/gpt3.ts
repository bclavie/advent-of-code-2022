const values = {
  "BX": 1,
  "CY": 2,
  "AZ": 3,
  "AX": 4,
  "BY": 5,
  "CZ": 6,
  "CX": 7,
  "AY": 8,
  "BZ": 9,
}

// Read the input file line by line
const readline = require('readline');
const rl = readline.createInterface({
  input: require('fs').createReadStream('input.txt')
});

let sum = 0;

// For each line, get the value and add it to the sum
rl.on('line', (line) => {
  const [letter1, letter2] = line.split(' ');
  const key = `${letter1}${letter2}`;
  const value = values[key];

  if (value) {
    sum += value;
  }
});

// Print the final sum when all lines have been processed
rl.on('close', () => {
  console.log(sum);
});
