# Puzzle 9: [Nine Eleven](https://i18n-puzzles.com/puzzle/9/)

To find the format used by each name, I go through each date and compute in which format this date would be valid. This actually requires to look at the number of days in each specific month, not doing just a generic `if days <= 31`.

The valid formats are returned as a bitmask. I put this bitmask in a map "name => formats". Next time a name is encountered again, the format mask I have so far is AND with the format generated for this day.

With the "name => formats" map, I can go through the list again and check which dates are 9/11 dates for the formats of those names. That gives me the list of names asked for.
