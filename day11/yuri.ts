import * as fs from 'fs';

interface turnResult {
    item: number;
    destinationMonkey: number;
}

class Monkey {
    private items: number[];
    private operation: (item: number) => number;
    private test: (item: number) => number;
    modulo: number;
    inspectCount: number = 0;

    turn(multipliedModulos: number | undefined): turnResult | undefined {
        let item = this.items.shift();
        if (item === undefined) {
            return undefined;
        }

        item = this.operation(item);
        this.inspectCount++;

        item = Math.floor(multipliedModulos === undefined ? item / 3 : item % multipliedModulos);

        let destinationMonkey = this.test(item);
        return {
            item: item,
            destinationMonkey: destinationMonkey
        }
    }

    receiveItem(item: number) {
        this.items.push(item);
    }

    constructor(
        startingItems: number[],
        modulo: number,
        operation: (item: number) => number,
        test: (item: number) => number) {
        this.items = startingItems;
        this.modulo = modulo;
        this.operation = operation;
        this.test = test;
    }
}

function runRounds(rounds: number, monkeys: Monkey[], multipliedModulos: number | undefined): number[] {
    for (let i = 0; i < rounds; i++) {
        for (let monkey of monkeys) {
            let turnResult = monkey.turn(multipliedModulos);

            while (turnResult !== undefined) {
                monkeys[turnResult.destinationMonkey].receiveItem(turnResult.item);
                turnResult = monkey.turn(multipliedModulos);
            }
        }
    }

    return monkeys.map(x => x.inspectCount).sort((a, b) => b - a);
}

function part1(monkeys: Monkey[]): number {
    let inspectionsByMonkey = runRounds(20, monkeys, undefined);
    return inspectionsByMonkey[0] * inspectionsByMonkey[1];
}

function part2(monkeys: Monkey[]): number {
    let multipliedModulo = monkeys.reduce((acc, val) => acc * val.modulo, 1);
    let inspectionsByMonkey = runRounds(10000, monkeys, multipliedModulo);

    return inspectionsByMonkey[0] * inspectionsByMonkey[1];
}

function parseMonkey(input: string[]): Monkey {
    let startingItems = input[1].split(":")[1].split(", ").map(x => parseInt(x));

    let operation = input[2].split("= ")[1];
    let operator = operation.indexOf("+") > 0 ? "+" : "*";
    let var1 = parseInt(operation.split(operator)[0].trim());
    let var2 = parseInt(operation.split(operator)[1].trim());
    let operationFn = (item: number) => {
        let p1 = (isNaN(var1) ? item : var1);
        let p2 = (isNaN(var2) ? item : var2);

        if (operator === "+") {
            return p1 + p2;
        };
        return p1 * p2;
    };

    let modulo = parseInt(input[3].split("by ")[1]);
    let monkeyIfTrue = parseInt(input[4].split("monkey")[1]);
    let monkeyIfFalse = parseInt(input[5].split("monkey")[1]);
    let testFn = (item: number) => { return (item % modulo === 0) ? monkeyIfTrue : monkeyIfFalse };

    return new Monkey(startingItems, modulo, operationFn, testFn);
}

fs.readFile('day11/input.txt', 'utf8', function (err, data) {
    let monkeys = data.replaceAll("\r", "").split("\n\n").map(x => x.split("\n")).map(x => parseMonkey(x));
    console.log(`Part 1: ${part1(monkeys)}`);

    monkeys = data.replaceAll("\r", "").split("\n\n").map(x => x.split("\n")).map(x => parseMonkey(x));
    console.log(`Part 2: ${part2(monkeys)}`);
});