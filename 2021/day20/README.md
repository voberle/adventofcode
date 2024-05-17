# Day 20: [Trench Map](https://adventofcode.com/2021/day/20)

## Part 1

This one had an interesting twist, with the test input working easily as expected, but not the real one. The real image had an algorith that would lit up all pixels with code 0, meaning one transformation resulted in an infinite number of pixels lit.

I got inially the answer by storing the pixels in a map, printing the image after two transformations and removing manually the extra pixels lit.

## Part 2

