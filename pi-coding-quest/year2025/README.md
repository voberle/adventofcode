# 2025 challenge

## Part 1

To manipulate the decimal prices without having round trouble, I used the `rust_decimal` crate and its `Decimal` type.

`fold()` made it easy to compute the code.

## Part 2

For deciphering the ticket, the code of the previous year was useful (but this time it was a right shift, sneaky!).

The only problem I had then was that I missed I had to re-sort the non-manipulated entries by day number.
