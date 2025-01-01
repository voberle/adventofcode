# Day 24: [Crossed Wires](https://adventofcode.com/2024/day/24)

## Part 1

Fairly simple gate simulation.

After an initial implementation using `String` and maps, I reimplemented it to refer to all gates by indexes only.
That was about 10 times faster for the execution part.

## Part 2

This one turned out to be hard work.

I ended up with two working solutions.

### Manual analysis + brute force

My initial idea was to use Graphviz to generate a visual representation of the circuit and find the errors there.

I used Gemini to help generate a Python script to convert the input into a graph.

    cat resources/input| python3 generate_graph.py > input.gv
    dot -Tpdf input.gv -o input.pdf -Kdot

The result is not perfect, but good enough. Unfortunately the graph was too big for me to find the swapped wires.

I did however observe that several Z wires were connected to the wrong gate type. This actually allowed me to identify 3 pairs of swapped wires.

For the last one, it was possible to brute-force it. The code supporting this investigation is in `circuit::swap_wires_investigation()`.

Unfortunately, due to a silly bug in how I set the initial wire values, the brute-forcing part didn't give me the correct pair initially. This made me believe that my 3 initial pairs were incorrect (they weren't), and I went looking for another method.

### Generating a working adder from scratch

The next idea I had was to generate a working adder circuit from scratch, then set all the known correct wire names in it, and deduct the swapped ones that way.

This worked out quite nicely actually. Generating the working adder circuit was fairly easy. I then used the fact that all gates with x and y as inputs were correct ones to deduct all the remaining ones. See in the `generate` module the details.