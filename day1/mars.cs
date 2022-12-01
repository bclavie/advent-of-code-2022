var input = File.ReadAllLines("input.txt");

var total = new List<int>();
int currentElf = 0;

foreach (var line in input)
{
    if (string.IsNullOrEmpty(line))
    {
        total.Add(currentElf);
        currentElf = 0;
    }
    else
    {
        currentElf += int.Parse(line);
    }
}

total.Add(currentElf);

Console.WriteLine("Part 1: The elf with the most calories has " + total.OrderByDescending(x => x).Take(1).Sum() + " calories.");

Console.WriteLine("Part 2: The three elves with the most calories have " + total.OrderByDescending(x => x).Take(3).Sum() + " calories.");