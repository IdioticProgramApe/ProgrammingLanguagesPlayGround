using System;

namespace FirstWinConsoleApplication
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Input a floating number: ");
            if (double.TryParse(Console.ReadLine(), out double input))
            {
                if (input >= 0)
                {
                    Console.WriteLine("{0}'s square root is {1}", input, Math.Sqrt(input));
                }
                else
                {
                    Console.WriteLine("{0} doesn't have a square root, because it's negative", input);
                }
            }
            else
            {
                Console.WriteLine("No floating point number is captured.");
            }
        }
    }
}
