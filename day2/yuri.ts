import * as fs from 'fs';

interface rockPaperScissorsRound {
    elfHandShape: string;
    playerHandShape: string;
}

interface roundScore {
    playerScore: number;
    elfScore: number;
}

function calculateRoundScorePart1(round: rockPaperScissorsRound): roundScore {
    let playerScore = round.playerHandShape.charCodeAt(0) - 23 - 64; // X = 1, Y = 2, Z = 3
    let elfScore = round.elfHandShape.charCodeAt(0) - 64; // A = 1, B = 2, C = 3

    if (playerScore === undefined || elfScore === undefined) {
        throw `You dun fucked up.`;
    }

    // draw
    if (playerScore === elfScore) {
        playerScore += 3;
        elfScore += 3;

        return { playerScore, elfScore };
    }

    // Shape score is Rock = 1, Paper = 2, Scissors = 3
    // Player wins if their shape score is greater than the Elf's while not scissors (3) vs rock (1). Or if they have rock (1) vs scissors (3)
    const isPlayerWin = (playerScore > elfScore && !(playerScore === 3 && elfScore === 1)) ||
        (playerScore === 1 && elfScore === 3);

    return {
        playerScore: playerScore + (isPlayerWin ? 6 : 0),
        elfScore: elfScore + (isPlayerWin ? 0 : 6)
    };
}

const losingMatchups = new Map<string, string>([
    ["A", "Z"],
    ["B", "X"],
    ["C", "Y"],
]);
const winningMatchups = new Map<string, string>([
    ["A", "Y"],
    ["B", "Z"],
    ["C", "X"],
]);

function calculateRoundScorePart2(round: rockPaperScissorsRound): roundScore {
    let playerHandShape: string | undefined;

    switch (round.playerHandShape) {
        case "X": // Lose
            playerHandShape = losingMatchups.get(round.elfHandShape);
            break;
        case "Y": // Draw
            playerHandShape = String.fromCharCode(round.elfHandShape.charCodeAt(0) + 23);
            break;
        case "Z": // Win
            playerHandShape = winningMatchups.get(round.elfHandShape);
            break;
    }

    if (playerHandShape === undefined) {
        throw `You dun fucked up.`;
    }

    return calculateRoundScorePart1({ playerHandShape, elfHandShape: round.elfHandShape });
}

function part1(rounds: rockPaperScissorsRound[]): number {
    return rounds
        .map(x => calculateRoundScorePart1(x))
        .reduce((accumulator, currentValue) => {
            return {
                elfScore: accumulator.elfScore + currentValue.elfScore,
                playerScore: accumulator.playerScore + currentValue.playerScore
            };
        }).playerScore;
}

function part2(rounds: rockPaperScissorsRound[]): number {
    return rounds
        .map(x => calculateRoundScorePart2(x))
        .reduce((accumulator, currentValue) => {
            return {
                elfScore: accumulator.elfScore + currentValue.elfScore,
                playerScore: accumulator.playerScore + currentValue.playerScore
            };
        }).playerScore;
}


fs.readFile('day2/input.txt', 'utf8', function (err, data) {
    const rounds: rockPaperScissorsRound[] = data
        .split("\n")
        .filter(x => x.trim().replace("\r", "").length > 1)
        .map(x => {
            let handShapes = x.replace("\r", "").split(" ");
            return { elfHandShape: handShapes[0], playerHandShape: handShapes[1] }
        });

    console.log(`Part 1: ${part1(rounds)}`);
    console.log(`Part 2: ${part2(rounds)}`);
});