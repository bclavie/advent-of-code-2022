import * as fs from 'fs';

const input: string  = fs.readFileSync('./day2/input.txt').toString();

const combinations = {
    'A X': 4,
    'A Y': 8,
    'A Z': 3,
    'B X': 1,
    'B Y': 5,
    'B Z': 9,
    'C X': 7,
    'C Y': 2,
    'C Z': 6,
  };

const plays = {
    'A X': 'Z',
    'A Y': 'X',
    'A Z': 'Y',
    'B X': 'X',
    'B Y': 'Y',
    'B Z': 'Z',
    'C X': 'Y',
    'C Y': 'Z',
    'C Z': 'X',
}

const solution = () => {
    const throws: string[] = input.split('\n');

    const firstSolution: number = throws.map(el => combinations[el]).reduce((a, b) => a + b)

    const secondSolution = throws.map((el, i) =>{
        const elfThrows: string = throws[i].substring(0, 1);
        const playerThrows: string = plays[el];

        return combinations[elfThrows + ' ' + playerThrows];
    }).reduce((a, b) => a + b);

    console.log(firstSolution);
    console.log(secondSolution);
}

solution();