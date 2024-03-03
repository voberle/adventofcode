# Day 4: [Secure Container](https://adventofcode.com/2019/day/4)

## Part 1

Fairly simple with iterator `windows` and reusing the `get_digits` from [2018 Day 14](../../2018/day14/README.md).

## Part 2

Ended up counting the number of consecutive digits in the number. Is it the best way? Nice thing is I got the method right on first try, zero bug!

## Optimization

There are ways to optimize it nicely, as shown in [this solution](https://gist.github.com/vlmonk/8f466c8b2d99c4bc0be3b355b783c44a):

Instead of converting each numbers to an array of digits, only convert the first number and then "increment" the array. The solution has also a neat trick to skip the numbers that would fit the "don't increase" rule.