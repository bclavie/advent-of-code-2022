import * as fs from 'fs';

fs.readFile('day1/input.txt', 'utf8', function (err, data) {
    const caloriesPerElf: number[] = [];
    
    data.split("\n")
        .map(x => parseInt(x))
        .reduce((accumulator, currentValue) => {
            if (Number.isNaN(currentValue)) {
                caloriesPerElf.push(accumulator)
                return 0;
            }

            return accumulator + currentValue;
        });

    console.log(`Part 1: ${part1(caloriesPerElf)}`);
    console.log(`Part 2: ${part2(caloriesPerElf)}`);
});

function part1(caloriesPerElf: number[]): number {
    return Math.max(...caloriesPerElf);
}

function part2(caloriesPerElf: number[]): number {
    return caloriesPerElf
        .sort((a, b) => b - a)
        .slice(0, 3)
        .reduce((accumulator, currentValue) => accumulator + currentValue);
}