import * as fs from 'fs';

interface sectionAssignment {
    fromId: number;
    toId: number;
}

function part1(sectionAssignments: sectionAssignment[][]): number {
    return sectionAssignments
        .reduce((accumulator, assignmentPair) => {
            if ((assignmentPair[0].fromId >= assignmentPair[1].fromId && assignmentPair[0].toId <= assignmentPair[1].toId) ||
                (assignmentPair[1].fromId >= assignmentPair[0].fromId && assignmentPair[1].toId <= assignmentPair[0].toId)) {
                return accumulator + 1;
            }

            return accumulator;
        }, 0);
}

function part2(sectionAssignments: sectionAssignment[][]): number {
    return sectionAssignments
        .reduce((accumulator, assignmentPair) => {
            if ((assignmentPair[0].fromId >= assignmentPair[1].fromId && assignmentPair[0].fromId <= assignmentPair[1].toId) ||
                (assignmentPair[1].fromId >= assignmentPair[0].fromId && assignmentPair[1].fromId <= assignmentPair[0].toId)) {
                return accumulator + 1;
            }

            return accumulator;
        }, 0);
}

fs.readFile('day04/input.txt', 'utf8', function (err, data) {
    let sectionAssignments: sectionAssignment[][] = data.split("\n")
        .map(x => x.split(",")
            .map(y => {
                let z = y.split("-");
                return {
                    fromId: parseInt(z[0]),
                    toId: parseInt(z[1])
                }
            }));

    console.log(`Part 1: ${part1(sectionAssignments)}`);
    console.log(`Part 2: ${part2(sectionAssignments)}`);
});