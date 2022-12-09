import * as fs from 'fs';

interface Motion {
    direction: string,
    steps: number
}
interface Position {
    x: number;
    y: number;
}

function part1(motions: Motion[]): number {
    let visitedPositions: Position[] = [];
    let headPosition: Position = { x: 0, y: 0 };
    let tailPosition: Position = { x: 0, y: 0 };

    for (var motion of motions) {
        for (var i = 0; i < motion.steps; i++) {
            // Move Head
            switch (motion.direction) {
                case "L":
                    headPosition.x -= 1;
                    break;
                case "R":
                    headPosition.x += 1;
                    break;
                case "U":
                    headPosition.y -= 1;
                    break;
                case "D":
                    headPosition.y += 1;
                    break;
            }

            // Move Tail
            let isTouching = Math.abs(headPosition.x - tailPosition.x) <= 1 && Math.abs(headPosition.y - tailPosition.y) <= 1;
            if (!isTouching) {
                // If on the same Y, move horizontally to close in
                if (headPosition.y == tailPosition.y) {
                    tailPosition.x += (tailPosition.x - headPosition.x < 0) ? 1 : -1;
                }
                // If on the same X, move vertically to close in
                else if (headPosition.x == tailPosition.x) {
                    tailPosition.y += (tailPosition.y - headPosition.y < 0) ? 1 : -1;
                }
                // If not on the same X or same Y, move diagonally towards head
                else {
                    tailPosition.x += (tailPosition.x - headPosition.x < 0) ? 1 : -1;
                    tailPosition.y += (tailPosition.y - headPosition.y < 0) ? 1 : -1;
                }
            }

            if (visitedPositions.findIndex(pos => pos.x == tailPosition.x && pos.y == tailPosition.y) === -1) {
                visitedPositions.push({ ...tailPosition });
            }
        }
    }
    return visitedPositions.length;
}

function part2(motions: Motion[]): number {
    let tailVisitedPositions: Position[] = [];
    let headKnot: Position = { x: 0, y: 0 };
    let tailKnot: Position = { x: 0, y: 0 };
    let rope = [headKnot, { x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 },
        { x: 0, y: 0 }, { x: 0, y: 0 }, { x: 0, y: 0 }, tailKnot];

    for (var motion of motions) {
        for (var i = 0; i < motion.steps; i++) {
            // Move Head
            switch (motion.direction) {
                case "L":
                    headKnot.x -= 1;
                    break;
                case "R":
                    headKnot.x += 1;
                    break;
                case "U":
                    headKnot.y -= 1;
                    break;
                case "D":
                    headKnot.y += 1;
                    break;
            };
            rope.forEach((knot, index) => {
                if (index === 0) return;

                let precedingKnot = rope[index - 1];
                let isTouching = Math.abs(precedingKnot.x - knot.x) <= 1 && Math.abs(precedingKnot.y - knot.y) <= 1;

                if (!isTouching) {
                    // If on the same Y, move horizontally to close in
                    if (precedingKnot.y == knot.y) {
                        knot.x += (knot.x - precedingKnot.x < 0) ? 1 : -1;
                    }
                    // If on the same X, move vertically to close in
                    else if (precedingKnot.x == knot.x) {
                        knot.y += (knot.y - precedingKnot.y < 0) ? 1 : -1;
                    }
                    // If not on the same X or same Y, move diagonally in preceding knot's direction
                    else {
                        knot.x += (knot.x - precedingKnot.x < 0) ? 1 : -1;
                        knot.y += (knot.y - precedingKnot.y < 0) ? 1 : -1;
                    }
                }

                if (index === 9 && tailVisitedPositions.findIndex(pos => pos.x == knot.x && pos.y == knot.y) === -1) {
                    tailVisitedPositions.push({ ...knot });
                }
            });
        }
    }
    return tailVisitedPositions.length;
}

fs.readFile('day09/input.txt', 'utf8', function (err, data) {
    let motions: Motion[] = data.replaceAll("\r", "").split("\n").map(x => {
        let parts = x.split(" ");
        return { direction: parts[0], steps: parseInt(parts[1]) };
    });

    console.log(`Part 1: ${part1(motions)}`);
    console.log(`Part 2: ${part2(motions)}`);
});