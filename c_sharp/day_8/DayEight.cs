using System.ComponentModel;

namespace c_sharp.day_8;

public class DayEight {
    public static void Run() { }

    public static int ProblemOne(string input) {
        return 0;
    }

    public static int ProblemTwo(string input) {
        return 0;
    }

    static List<List<int>> ParseTrees(string input) {
        var trees = input
            .Lines()
            .Select(line => line
                .ToCharArray()
                .Select(c => int.Parse(c.ToString()))
                .ToList())
            .ToList();

        return trees;
    }

    public static void Test() {
        const string input = @"30373
25512
65332
33549
35390";

        var expected = 21;
        var actual = ProblemOne(input);
        Console.WriteLine($"Problem 1 passed: {expected == actual}\n expected: {expected}, actual: {actual}");

        expected = 8;
        actual = ProblemTwo(input);
        Console.WriteLine($"Problem 2 passed: {expected == actual}\n expected: {expected}, actual: {actual}");
    }
}
