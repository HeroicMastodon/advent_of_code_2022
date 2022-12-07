
namespace c_sharp.day_4;

record Range(int Lower, int Upper) {
    public static Range From(string input) {
        var split = input.Split('-').Select(int.Parse).ToList();
        return new Range(split[0], split[1]);
    }

    public bool Contains(Range other) => Lower <= other.Lower && Upper >= other.Upper;

    public bool Overlaps(Range other) =>
        (Lower >= other.Lower && Lower <= other.Upper) || (Upper <= other.Upper && Upper >= other.Lower);
}

public static class DayFour {
    public static void Run() {
        ProblemOne();
        ProblemTwo();
    }

    public static void ProblemOne() {
        var input = InputFinder.GetInputForDay(4);
        var result = input.Lines().Aggregate(0, ( agg, line ) => {
            var split = line.Split(',');
            var first = Range.From(split[0]);
            var second = Range.From(split[1]);

            if (first.Contains(second) || second.Contains(first)) return agg + 1;

            return agg;
        });
        
        Console.WriteLine(result);
    }

    public static void ProblemTwo() {
        var input = InputFinder.GetInputForDay(4);
        var result = input.Lines().Aggregate(0, ( agg, line ) => {
            var split = line.Split(',');
            var first = Range.From(split[0]);
            var second = Range.From(split[1]);

            if (first.Overlaps(second) || second.Overlaps(first)) return agg + 1;

            return agg;
        });
        
        Console.WriteLine(result);
    }
}
