import * as fs from 'fs';

function part1(treeHeights: number[][]): number {
    let visibleCount = treeHeights[0].length * 2 + (treeHeights.length - 2) * 2;
    for (let y = 1; y < treeHeights.length - 1; y++) {
        for (let x = 1; x < treeHeights[y].length - 1; x++) {
            let currentHeight = treeHeights[y][x];
            let arrLeft = treeHeights[y].slice(0, x);
            let arrRight = treeHeights[y].slice(x + 1, treeHeights[y].length);
            let arrTop = Array.from(Array(y).keys()).map(z => treeHeights[z][x]);
            let arrBottom = Array.from(Array(treeHeights.length - y - 1).keys()).map(z => treeHeights[y + z + 1][x]);

            if (arrLeft.every(h => h < currentHeight) ||
                arrRight.every(h => h < currentHeight) ||
                arrTop.every(h => h < currentHeight) ||
                arrBottom.every(h => h < currentHeight)) {
                visibleCount++;
            }
        }
    };
    return visibleCount;
}

function part2(treeHeights: number[][]): number {
    let highestScenicScore = 0;
    for (let y = 0; y < treeHeights.length; y++) {
        for (let x = 0; x < treeHeights[y].length; x++) {
            let currentHeight = treeHeights[y][x];
            let arrLeft = treeHeights[y].slice(0, x);
            let arrRight = treeHeights[y].slice(x + 1, treeHeights[y].length);
            let arrTop = Array.from(Array(y).keys()).map(z => treeHeights[z][x]);
            let arrBottom = Array.from(Array(treeHeights.length - y - 1).keys()).map(z => treeHeights[y + z + 1][x]);

            highestScenicScore = Math.max(getScenicScore(currentHeight, arrLeft.reverse()) * getScenicScore(currentHeight, arrRight) * getScenicScore(currentHeight, arrTop.reverse()) * getScenicScore(currentHeight, arrBottom), highestScenicScore);
        }
    };

    return highestScenicScore;
}

function getScenicScore(currentHeight: number, collidingTrees: number[]): number {
    if (collidingTrees.length < 1) {
        return 0;
    }

    let distance = collidingTrees.findIndex(x => x >= currentHeight);
    return distance === -1 ? collidingTrees.length : distance + 1;
}

fs.readFile('day08/input.txt', 'utf8', function (err, data) {
    let treeHeights = data.replaceAll("\r", "").split("\n").map(x => [...x].map(y => parseInt(y)));

    console.log(`Part 1: ${part1(treeHeights)}`);
    console.log(`Part 2: ${part2(treeHeights)}`);
});