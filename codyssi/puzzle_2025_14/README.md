# Puzzle 14: [Crucial Crafting!](https://www.codyssi.com/view_problem_18?)

Part 1 was quite easy.

Part 2 was brute-forced in a recursive way: We go through items starting with highest quality and try to find the best combination.

Part 3 couldn't be brute-forced. As I didn't know the algorithm to use, I asked Gemini for which one to use.

My question was:

    If I have a set of items that have a certain value and a certain cost each. I want to find the combination of items that maximises the value while keeping the total cost under a given number. What would be an approach to do this?

The answer:

    This is a classic optimization problem known as the Knapsack Problem. Specifically, it sounds like you're describing the 0/1 Knapsack Problem, where you can either take an item in its entirety (1) or not at all (0).

From there, the implementation was fairly simple.
