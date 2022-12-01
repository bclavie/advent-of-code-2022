import * as fs from 'fs';

const input: string  = fs.readFileSync('./day1/input.txt').toString();

const solution = () => {
    const bags: any = input.split('\n\n');

    let bagArray: any = bags.reduce((arr, el,) => {
        arr.push(el.split('\n').map(x => parseInt(x)))
        
        return arr;
    }, []);

    const sumOfSnacks = bagArray.reduce((arr, el) => {
        arr.push(el.reduce((a, b) => a + b, 0))
        
        return arr;
    }, []);

    const firstSolution: number = Math.max(...sumOfSnacks);
    const secondSolution: number = sumOfSnacks.sort((a,b) => b - a).slice(0, 3).reduce((a, b) => a + b, 0);

    console.log(firstSolution);
    console.log(secondSolution);
}

solution();
