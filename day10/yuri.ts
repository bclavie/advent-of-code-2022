import * as fs from 'fs';

function part1(input: string[]): number {
    let cycles: number[] = [];
    let x = 1;
    cycles.push(x);

    input.forEach(command => {
        let arr = command.replaceAll("\r", "").split(" ");
        let operation = arr[0];
        let val = parseInt(arr[1]);

        if (operation === "noop") {
            cycles.push(x);
        }

        if (operation === "addx") {
            cycles.push(x);
            cycles.push(x);
            x += val;
        }
    });

    return [20, 60, 100, 140, 180, 220].reduce((acc, val) => acc += cycles[val] * val, 0);
}

function part2(input: string[]): string {
    let cycles: number[] = [];
    let crt: string[][] = Array.from(Array(7).keys()).map(_ => Array.from(Array(40).keys()).map(_ => "."));
    let crtPosition: number = 0;
    let x = 1;
    cycles.push(x);

    const cycle = () => {
        cycles.push(x);
        crtCycle();
    }

    const crtCycle = () => {
        let crtLine = Math.floor(cycles.length / 40);
        let latestX = cycles[cycles.length - 1];
        let spritePosition: number[] = [latestX - 1, latestX, latestX + 1];

        crt[crtLine][crtPosition % 40] = spritePosition.includes(crtPosition % 40) ? "#" : ".";
        crtPosition += 1;
    }

    input.forEach(command => {
        let arr = command.replaceAll("\r", "").split(" ");
        let operation = arr[0];
        let val = parseInt(arr[1]);

        if (operation === "noop") {
            cycle();
        }

        if (operation === "addx") {
            cycle();
            cycle();
            x += val;
        }
    });

    return crt.slice(0, 6).map(line => line.join("") + "\n").join("");
}

fs.readFile('day10/input.txt', 'utf8', function (err, data) {
    console.log(`Part 1: ${part1(data.split("\n"))}`);
    console.log(`Part 2: \n${part2(data.split("\n"))}`);
});