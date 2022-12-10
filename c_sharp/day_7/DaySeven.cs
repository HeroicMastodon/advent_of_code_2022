namespace c_sharp.day_7;

public static class DaySeven {
    public static void Run() {
        var input = InputFinder.GetInputForDay(7);
        Console.WriteLine(ProblemOne(input));
        Console.WriteLine(ProblemTwo(input));
    }

    public static int ProblemOne(string input) {
        var sizes = CalculateDirectorySizes(input);

        return sizes.Values.Sum(size => size <= 100000 ? size : 0);
    }

    public static int ProblemTwo(string input) {
        var sizes = CalculateDirectorySizes(input);
        const int totalSpace = 70000000;
        const int updateSize = 30000000;
        var remainingSpace = totalSpace - sizes["/"];
        var neededSpace = updateSize - remainingSpace;
        var result = sizes.Values.Where(size => size >= neededSpace).Min();

        return result;
    }

    static Dictionary<string, int> CalculateDirectorySizes(string input) {
        var dirSizes = new Dictionary<string, int>();
        var currDir = "";
        var dirStack = new LinkedList<string>();

        foreach (var line in input.Lines()) {
            var split = line.Split(" ");

            var first = split.FirstOrDefault();
            var second = split.Skip(1).FirstOrDefault();
            var third = split.Skip(2).FirstOrDefault();

            if (first == "$" && second == "cd") {
                if (third == "..") {
                    currDir = dirStack.Last?.Value ?? "";
                    dirStack.RemoveLast();
                    continue;
                }

                if (third == "/") {
                    currDir = "/";
                    dirSizes["/"] = 0;
                    dirStack.Clear();
                }

                dirStack.AddLast(currDir);
                currDir = $"{currDir}/{third}";
            }
            else if (first == "$" && second == "ls") { }
            else if (first != "dir") {
                if (!dirSizes.ContainsKey(currDir)) {
                    dirSizes.Add(currDir, 0);
                }

                var dirSize = int.Parse(first ?? "0");
                dirSizes[currDir] += dirSize;

                foreach (var dir in dirStack) {
                    dirSizes[dir] += dirSize;
                }
            }
        }

        return dirSizes;
    }

    public static void Test() {
        const string input = @"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        var expected = 95437;
        Console.WriteLine($"problem 1:\n expected: {expected}, actual: {ProblemOne(input)}");
        expected = 24933642;
        Console.WriteLine($"problem 2:\n expected: {expected}, actual: {ProblemTwo(input)}");
    }
}
