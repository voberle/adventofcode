# Day 3: [Toboggan Trajectory](https://adventofcode.com/2020/day/3)

## Part 1

Nothing complicated here, reusing my Grid code, and the infinite aspect is easy to handle.

## Part 2

Simple extension of part 1.

## Update

On Reddit saw [a beautiful Rust solution](https://www.reddit.com/r/adventofcode/comments/k5qsrk/comment/geh0gqn/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button):

    fn trees(grid: &str, ic: usize, il: usize) -> usize {
        grid.lines()
            .step_by(il)
            .enumerate()
            .filter(|(step, l)| l.as_bytes()[(*step * ic) % l.len()] == b'#')
            .count()
    }