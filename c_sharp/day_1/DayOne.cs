namespace c_sharp.day_1;

public static class DayOne {
    public static void ProblemOne() {
        var input = InputFinder.GetInputForDay(1);
        var summed = input
            .Split("\n\n")
            .Select(group => group
                .Lines()
                .Sum(food => uint.Parse(food))
            )
            .ToList();
        
        summed.Sort();
        var result = summed.Last();
        
        Console.WriteLine(result);
    }

    public static void ProblemTwo() {
        var input = InputFinder.GetInputForDay(1);
        var summed = input
            .Split("\n\n")
            .Select(group => group
                .Lines()
                .Sum(food => uint.Parse(food))
            )
            .ToList();
        
        summed.Sort();
        var result = summed.TakeLast(3).Sum();
        
        Console.WriteLine(result);
    }
}
