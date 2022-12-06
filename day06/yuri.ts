import * as fs from 'fs';

function part1(data: string): number {
    return indexOfFirstUniqueSequence(data, 4);
}

function part2(data: string): number {
    return indexOfFirstUniqueSequence(data, 14);
}

function indexOfFirstUniqueSequence(data: string, sequenceLength: number): number {
    let buffer: string[] = [];
    for (let [index, character] of [...data].entries()) {
        if (buffer.length > (sequenceLength - 1)) {
            buffer.shift();
        }

        buffer.push(character);

        if (new Set(buffer).size === sequenceLength) {
            return index + 1;
        }
    }
    return -1;
};

fs.readFile('day06/input.txt', 'utf8', function (err, data) {
    console.log(`Part 1: ${part1(data)}`);
    console.log(`Part 2: ${part2(data)}`);
});