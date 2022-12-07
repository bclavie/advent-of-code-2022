import * as fs from 'fs';

interface Node {
    key: string;
    size: number;
    children?: Node[];
    parent?: Node;
}

function part1(rootNode: Node): number {
    return findDirectoryNodes(node => node.size < 100000, rootNode)
        .reduce((acc, val) => acc + val.size, 0);
}

function part2(rootNode: Node): number {
    let minFolderSize = Math.abs(70000000 - 30000000 - rootNode.size);
    return findDirectoryNodes(x => x.size > minFolderSize, rootNode)
        .sort((x, y) => x.size - y.size)[0].size;
}

function findDirectoryNodes(condition: (node: Node) => boolean, currentNode: Node): Node[] {
    if (!currentNode.children) { // is file
        return [];
    }

    let matchingChildNodes = (currentNode.children.flatMap(child => findDirectoryNodes(condition, child)));
    return (condition(currentNode) ? [currentNode] : []).concat(matchingChildNodes);
}

function setSizeRecursive(node: Node): number {
    if (node.children === undefined || node.children.length === 0) {
        return node.size || 0;
    }

    node.size = node.children.map(c => setSizeRecursive(c)).reduce((accumulator, childSize) => accumulator + childSize);
    return node.size;
}

function parseTree(commands: string[]): Node {
    let rootNode = { key: "/", size: 0 }
    let currentNode: Node = rootNode;

    for (let cmd of commands.slice(1)) {
        if (cmd.startsWith("$ cd")) {
            currentNode = cd(cmd, currentNode);
            continue;
        }

        if (cmd === "$ ls") {
            continue;
        }

        createChildNode(cmd, currentNode);
    };
    setSizeRecursive(rootNode);
    return rootNode;
}

function createChildNode(command: string, currentNode: Node): Node {
    let cmd = command.split(" ");

    if (!currentNode.children) {
        currentNode.children = [];
    }

    if (cmd[0] === "dir") {
        currentNode.children.push({ key: cmd[1], parent: currentNode, size: 0 });
    }
    else {
        currentNode.children.push({ size: parseInt(cmd[0]), key: cmd[1], parent: currentNode }); // push file
    }

    return currentNode.children[currentNode.children.length - 1];
}

function cd(command: string, currentNode: Node): Node {
    let key = command.split(" ")[2];

    if (key === "..") {
        return currentNode.parent ?? currentNode; // return parent if exists
    }

    let child = currentNode.children?.find(x => x.key === key);
    if (!child) {
        throw new Error(`Couldn't find child node of key ${key}`);
    }

    return child;
}

fs.readFile('day07/input.txt', 'utf8', function (err, data) {
    let rootNode = parseTree(data.replaceAll("\r", "").split("\n"));

    console.log(`Part 1: ${part1(rootNode)}`);
    console.log(`Part 2: ${part2(rootNode)}`);
});