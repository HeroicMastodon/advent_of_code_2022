namespace c_sharp.day_3;

public class DayThree {
    public static void Run() {
        ProblemOne();
        ProblemTwo();
    }
    public static void ProblemOne() {
        var input = InputFinder.GetInputForDay(3);
        var result = input.Lines()
            .Select(line => {
                var chars = line.ToCharArray();
                var size = chars.Length / 2;
                var first = chars.Take(size).ToHashSet();
                var second = chars.Skip(size).ToHashSet();
                first.IntersectWith(second);

                return Prioritize(first.First());
            })
            .Sum();

        Console.WriteLine(result);
    }

    public static void ProblemTwo() {
        var input = InputFinder.GetInputForDay(3);
        var result = input.Lines()
            .Chunk(3)
            .Select(group => Prioritize(group.Aggregate(new HashSet<char>(),
                    (set, s) => {
                        if (set.Count == 0) return s.ToHashSet();
                        set.IntersectWith(s.ToHashSet());
                        return set;
                    })
                .First()))
            .Sum();

        Console.WriteLine(result);
    }

    static int Prioritize(char c) => c is >= 'a' and <= 'z' ? c - 'a' + 1 : c - 'A' + 27;
}
