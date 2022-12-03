import * as fs from 'fs';

function getItemPriority(item: string): number {
    var priority = item.charCodeAt(0) - 96;
    if (priority < 1) {
        priority += 58;
    }
    return priority;
}

function part1(itemPrioritiesByCompartment: { compartment1: number[], compartment2: number[] }[]): number {
    return itemPrioritiesByCompartment
        .map(x => x.compartment1.find(y => x.compartment2.indexOf(y) >= 0) ?? 0)
        .reduce((accumulator, currentValue) => accumulator + currentValue);
}

function part2(itemPrioritiesByGroup: number[][][]): number {
    return itemPrioritiesByGroup
        .map(group => group[0].find(x => group[1].indexOf(x) >= 0 && group[2].indexOf(x) >= 0) ?? 0)
        .reduce((accumulator, currentValue) => accumulator + currentValue);
}

fs.readFile('day3/input.txt', 'utf8', function (err, data) {
    let itemPrioritiesByCompartment = data
        .split("\n")
        .map(elfItems => ({
            compartment1: [...elfItems.slice(0, elfItems.length / 2)].map(compartmentItem => getItemPriority(compartmentItem)),
            compartment2: [...elfItems.slice(elfItems.length / 2)].map(compartmentItem => getItemPriority(compartmentItem))
        }));

    console.log(`Part 1: ${part1(itemPrioritiesByCompartment)}`);

    const numElvesPerGroup = 3;
    let itemPrioritiesByGroup = data
        .split("\n")
        .reduce((accumulator, currentValue, index) => {
            let groupNum = Math.floor(index / numElvesPerGroup);

            if (!accumulator[groupNum]) {
                accumulator[groupNum] = [] as number[][];
            }

            accumulator[groupNum].push([...currentValue].map(groupItem => getItemPriority(groupItem)));
            return accumulator;
        }, [] as number[][][]);

    console.log(`Part 2: ${part2(itemPrioritiesByGroup)}`);
});