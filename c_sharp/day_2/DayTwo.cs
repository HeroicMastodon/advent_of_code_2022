namespace c_sharp.day_2;

public static class DayTwo {
    private const int ROCK = 1;
    private const int PAPER = 2;
    private const int SCISSORS = 3;
    private const int LOSS = 0;
    private const int DRAW = 3;
    private const int WIN = 6;

    public static void ProblemOne() {
        var input = InputFinder.GetInputForDay(2);
        var result = input.Lines().Sum(SumGameOne);
        Console.WriteLine(result);
    }

    public static void ProblemTwo() {
        var input = InputFinder.GetInputForDay(2);
        var result = input.Lines().Sum(SumGameTwo);
        Console.WriteLine(result);
    }

    static int SumGameOne(string game) {
        return game switch {
            // Rock
            "A X" => ROCK + DRAW,
            "A Y" => PAPER + WIN,
            "A Z" => SCISSORS + LOSS,
            // Paper
            "B X" => ROCK + LOSS,
            "B Y" => PAPER + DRAW,
            "B Z" => SCISSORS + WIN,
            // Scissors
            "C X" => ROCK + WIN,
            "C Y" => PAPER + LOSS,
            "C Z" => SCISSORS + DRAW,
            _ => throw new ArgumentOutOfRangeException(nameof(game), game, null)
        };
    }

    static int SumGameTwo(string game) {
        return game switch {
            // Rock
            "A X" => SCISSORS + LOSS,
            "A Y" => ROCK + DRAW,
            "A Z" => PAPER + WIN,
            // Paper
            "B X" => ROCK + LOSS,
            "B Y" => PAPER + DRAW,
            "B Z" => SCISSORS + WIN,
            // Scissors
            "C X" => PAPER + LOSS,
            "C Y" => SCISSORS + DRAW,
            "C Z" => ROCK + WIN,
            _ => throw new ArgumentOutOfRangeException(nameof(game), game, null)
        };
    }
}
