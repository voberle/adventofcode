# Puzzle 15: [24/5 support](https://i18n-puzzles.com/puzzle/15/)

This one took me some time, even if the final solution isn't that complicated.

The parsing already wasn't completely trivial. To split the 3 fields, I used the fact that only the timezones contain a '/' and that they have no spaces.

I parse the timezone as a `chrono_rz::Tz` and the public holidays as a vector of `NaiveDate`.

Then to find the answer:

For each customer, I go through all the minutes of the year, in UTC. 

For each minute, I convert it into the customer timezone and check if it's a working day (Mon to Fri) and not a public holiday.

If that's true, I go through all offices and if none is working, I count an overtime minute.

To determine if an office is working, I convert the time into the office timezone and check if it's a working hour, a working day and not a public holiday.

Since I do the check minute by minute, it was slow, 3.2 seconds.

I optimized it to calculate first the office availability for each minute, which brought the time down to 1.3 second.

But the real speed up was going by steps of 30 minutes instead of 1 minute: Now it runs in 50 ms.
