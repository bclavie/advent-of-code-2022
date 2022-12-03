string data = ("C:/Users/Hazel/source/repos/adventcod3/adventcod3/bin/input.txt");
StreamReader newreader = new StreamReader(data);
StreamReader threereader = new StreamReader(data);

List<string> part2 = new List<string>();
List<char> part2sol = new List<char>();
List<char> items = new List<char>();
List<double> count = new List<double>();
List<double>count1 = new List<double>();
char[] alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".ToCharArray();


string line = " ";
string line1 = " ";

while (line != null)
{
    line = newreader.ReadLine();
    if (string.IsNullOrEmpty(line))
    { break; }
    else
    {
        var first = line.Substring(0, (int)line.Length / 2);
        var last = line.Substring((int)line.Length / 2, (int)(line.Length / 2));
        var common = first.Intersect(last);
        foreach (var c in common)
        {
            items.Add(c);
        }
    }
}
foreach (var item in items)
{
    var index = Array.FindIndex(alphabet, c => c.Equals(item));
    count.Add(index + 1);
}

for (int i = 0; i < 100; i++)
{
    for (int j = 0; j < 3; j++)
    {
        line1 = threereader.ReadLine();
        part2.Add(line1);
    }
    string first = part2[0];
    string second = part2[1];
    string third = part2[2];
    var common = first.Intersect(second);
    var common2 = second.Intersect(third);
    var commoner = common.Intersect(common2);
    foreach (var c in commoner)
    {
        part2sol.Add(c);
    }
    part2.Clear();
}
foreach (var item in part2sol)
{
    var index = Array.FindIndex(alphabet, c => c.Equals(item));
    count1.Add(index + 1);
}
double total = count.Sum();
double total1 = count1.Sum();
Console.WriteLine("{0},{1}",total,total1);
Console.ReadLine();