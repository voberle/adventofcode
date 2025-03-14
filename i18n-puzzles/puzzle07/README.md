# Puzzle 7: [The audit trail fixer](https://i18n-puzzles.com/puzzle/7/)

First I parse the timestamp string into a `DateTime<FixedOffset>` with  `DateTime::parse_from_rfc3339()`.

Then I'm using the `chrono_tz` crate to get the timezone data for the two cities and I convert that datetime into `DateTime<Tz>`:

    let halifax_time = self.datetime.with_timezone(&Tz::America__Halifax);

And I extract the Daylight Saving Time offset with:

    let halifax_dst_offset = halifax_time.offset().dst_offset();

By comparing this offset with the offset char that is in the timestamp string, I can deduce which city we use, for example:

    // In Halifax, during winter, DST offset is 0 and time string ends with 4.
    if (halifax_dst_offset == TimeDelta::zero() && offset_char == '4') {
        // Halifax

Getting the correct time means then taking that `DateTime<Tz>` adding the the correct duration and removing the incorrect one.