namespace c_sharp.day_6;

public class DaySix {
    static int FindUnique(string input, int count) {
        var index = 0;
        var deck = new LinkedList<char>();
        foreach (var c in input.ToCharArray()) {
            index++;
            deck.AddLast(c);
            var set = deck.ToHashSet();
            
            if (set.Count == count) {
                return index;
            }

            if (set.Count != deck.Count) {
                deck.RemoveFirst();
            }
        }

        return index;
    }
    
    public static void Run() {
        ProblemOne();
        ProblemTwo();
    }

    public static void ProblemOne() {
        var input = InputFinder.GetInputForDay(6);
        Console.WriteLine(FindUnique(input, 4));
    }

    public static void ProblemTwo() {
        var input = InputFinder.GetInputForDay(6);
        Console.WriteLine(FindUnique(input, 14));
    }

    public static void test() {
        const string input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        var expected = 7;
        var actual = FindUnique(input, 4);
        Console.WriteLine(expected == actual);
        
        expected = 19;
        actual = FindUnique(input, 14);
        Console.WriteLine(expected == actual);
    }
}
