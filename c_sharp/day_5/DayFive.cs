namespace c_sharp.day_5;

record Move(int Count, int From, int To) {
    public static Move FromString(string input) {
        var split = input.Split(" ").ToList();
        return new Move(int.Parse(split[1]), int.Parse(split[3]) - 1, int.Parse(split[5]) - 1);
    }
};

record ParseResult(List<Move> Moves, List<List<char>> Crates);

public static class DayFive {
    public static void Run() {
        ProblemOne();
        ProblemTwo();
    }

    static ParseResult Parse(string input) {
        var split = input.Split("\n\n").ToList();
        return new ParseResult(ParseMoves(split[1]).ToList(), ParseCrates(split[0]));
    }

    static List<List<char>> ParseCrates(string input) {
        var lines = input.Lines().ToList();
        var crates = new List<List<char>>();
        
        foreach (var line in lines) {
            if (crates.Count == 0) {
                for (var i = 0; i <=  line.Length/4; i++) {
                    crates.Add(new List<char>());
                }
            }

            var chars = line.ToCharArray();
            foreach (var stack in Enumerable.Range(0, crates.Count)) {
                foreach (var c in chars.Take(4)) {
                    if (char.IsLetter(c)) {
                        crates[stack].Add(c);
                    }
                }

                chars = chars.Skip(4).ToArray();
            }
        }
        
        foreach (var crate in crates) {
            crate.Reverse();
        }

        return crates;
    }

    static IEnumerable<Move> ParseMoves(string input) {
        var lines = input.Lines().ToList();
        return lines.Select(Move.FromString);
    }

    public static void ProblemOne() {
        var input = InputFinder.GetInputForDay(5);
        var (moves, crates) = Parse(input);
        
        foreach (var (count, from, to) in moves) {
            foreach (var _ in Enumerable.Range(0, count)) {
                var fromStack = crates[from];
                var c = fromStack.Last();
                fromStack.RemoveAt(fromStack.LastIndexOf(c));
                
                crates[to].Add(c);
            }
        }
        
        var result = string.Join("", crates.Select(stack => stack.Last()));
        Console.WriteLine(result);
    }

    public static void ProblemTwo() {
        var input = InputFinder.GetInputForDay(5);
        var (moves, crates) = Parse(input);
        
        foreach (var (count, from, to) in moves) {
            var toMove = new List<char>();
            foreach (var _ in Enumerable.Range(0, count)) {
                var fromStack = crates[from];
                var c = fromStack.Last();
                fromStack.RemoveAt(fromStack.LastIndexOf(c));
                
                toMove.Add(c);
            }
            toMove.Reverse();
            crates[to].AddRange(toMove);
        }
        
        var result = string.Join("", crates.Select(stack => stack.Last()));
        Console.WriteLine(result);
    }
}
