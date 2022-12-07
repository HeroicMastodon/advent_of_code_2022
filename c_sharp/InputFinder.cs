namespace c_sharp;

public static class InputFinder {
    public static string GetInputForDay(int day) => File.ReadAllText($"../../../day_{day}/input.txt");
}

public static class StringExtensions {
    public static IEnumerable<string> Lines(this string val) {
        return val.Split("\n");
    }
}
