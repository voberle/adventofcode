# Day 20: [Trench Map](https://adventofcode.com/2021/day/20)

## Part 1

This one had an interesting twist, with the test input working easily as expected, but not the real one. The real image had an algorith that would lit up all pixels with code 0, meaning one transformation resulted in an infinite number of pixels lit.

I got inially the answer by storing the pixels in a map, printing the image after two transformations and removing manually the extra pixels lit.

Using an actual grid may have been an option, but transforming it wasn't that easy. Growing the grid by one row/col on each iteration with by single vector backed grid is a bit painful to do.

So to fix the solution, I added border support to the image: When calculating the code for a pixel, we check if the pixel is within the current borders of the image. If it isn't, we use a default value for it. For the real picture, the default value is flipped on each transformation.

## Part 2

For 50 transformations, I had to refactor things a bit to avoid recalculating the borders of the image too often.

Both parts run in 75 ms.
