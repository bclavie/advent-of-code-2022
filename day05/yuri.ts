import * as fs from 'fs';

interface Command {
    moveCount: number;
    moveFrom: number;
    moveTo: number;
}

function part1(stack: string[][], commands: Command[]): string {
    commands.forEach(command =>
        Array.from({ length: command.moveCount })
            .forEach(_ => stack[command.moveTo].push(stack[command.moveFrom].pop() ?? ""))
    );

    return stack.map(x => x.pop()).join("");
}

function part2(stack: string[][], commands: Command[]): string {
    commands.forEach(command => stack[command.moveTo].push(...stack[command.moveFrom].splice(stack[command.moveFrom].length - command.moveCount, command.moveCount)));
    return stack.map(x => x.pop()).join("");
}

function parseStartingStack(data: string[]): string[][] {
    let stackLinesCount = data.findIndex(x => x.startsWith(" 1"));
    let stack: string[][] = [];
    
    for (let i = 0; i < stackLinesCount; i++) {
        let line = [...data[i].replaceAll(/\s?\[|\]\s|\r/ig, "").replaceAll("   ", "-").replaceAll(" ", "")];
        line.forEach((val, index) => {
            if (val === "-") {
                return;
            }
            stack[index] = [val].concat(stack[index] ?? []);
        });
    }
    return stack;
}

function parseCommands(data: string[]): Command[] {
    return data.slice(data.findIndex(x => x.startsWith("move")))
        .map(x => {
            let line = x.replace(/move\s|from\s|to\s|\r/ig, "").split(" ");
            return { moveCount: parseInt(line[0]), moveFrom: parseInt(line[1]) - 1, moveTo: parseInt(line[2]) - 1 };
        });
}

fs.readFile('day05/input.txt', 'utf8', function (err, data) {
    let stack = parseStartingStack(data.split("\n"));
    let commands = parseCommands(data.split("\n"));

    console.log(`Part 1: ${part1(JSON.parse(JSON.stringify(stack)), commands)}`);
    console.log(`Part 2: ${part2(JSON.parse(JSON.stringify(stack)), commands)}`);
});