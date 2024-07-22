# `sqlite-jiff`

Work-in-progress date-time SQLite extension that will support timezones, complex durations, and daylight savings calculations, based on the [`jiff` library](https://github.com/BurntSushi/jiff).

```sql
.load ./jiff0

select jiff_duration(
  '2024-11-02T01:59:59[America/Los_Angeles]',
  '2024-11-02T02:00:01[America/New_York]',
  'minutes'
) as result;
/*
┌──────────────────┐
│      result      │
├──────────────────┤
│ 179.966666666667 │
└──────────────────┘
*/

select jiff_duration(
  '2024-11-03T01:59:59[America/Los_Angeles]',
  '2024-11-03T02:00:01[America/New_York]',
  'minutes'
) as result;
/*
┌──────────────────┐
│      result      │
├──────────────────┤
│ 119.966666666667 │
└──────────────────┘
*/
```

Note that in this example, `2024-11-03` at 2AM is Daylight Savings "fall back", which results in a 2hr difference instead of the "normal" 3hr difference.

Mostly a "this can work" project, not usable for most people.

If there's a feature or use-case that you think would be interesting in this extension, feel free to file an issue! I personally won't work much on this extension until `sqlite-vec` is out.
