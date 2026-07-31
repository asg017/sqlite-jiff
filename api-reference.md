


## Time

"Time" in `sqlite-jiff` refers to "wall clock" time, like `11:30:52` `23:59:59.9999`, and `00:00:01`. Times are comprised of hour, minute, second, and optional sub-second components.

Time references are internally backed by [`jiff::civil::Time`](https://docs.rs/jiff/latest/jiff/struct.Span.html).

### Time Constructors

#### `jiff_time()` {#jiff_time}

```sql
select jiff_time('08:59:50');
-- '08:59:50'
select jiff_time('08:59:50.123');
-- '08:59:50.123'
select jiff_time('23:59:59.9999');
-- '23:59:59.9999'
```

```sql
select jiff_time(8, 59, 59);
-- '08:59:59'
select jiff_time(8, 59, 59, 1_000_000);
-- '08:59:59.001'
select jiff_time(8, 59, 59, 11_000_000);
-- '08:59:59.011'
select jiff_time(8, 59, 59, 111_000_000);
-- '08:59:59.111'
```

#### `jiff_time_midnight()` {#jiff_time_midnight}

```sql
```


#### `jiff_time_strptime()` {#jiff_time_strptime}

```sql
```

### Time Components

#### `jiff_time_hour()` {#jiff_time_hour}

```sql
```

#### `jiff_time_minute()` {#jiff_time_minute}

```sql
```

#### `jiff_time_second()` {#jiff_time_second}

```sql
```

#### `jiff_time_millisecond()` {#jiff_time_millisecond}

```sql
```

#### `jiff_time_nanosecond()` {#jiff_time_nanosecond}

```sql
```

#### `jiff_time_microsecond()` {#jiff_time_microsecond}

```sql
```

### Time Utilities


#### `jiff_time_round()` {#jiff_time_round}

```sql
```

#### `jiff_time_strftime()` {#jiff_time_strftime}

```sql
```

### Time Arithmetic

#### `jiff_time_add()` {#jiff_time_add}

```sql
```

#### `jiff_time_sub()` {#jiff_time_sub}

```sql
```


#### `jiff_time_add_wrapping()` {#jiff_time_add_wrapping}

```sql
```

#### `jiff_time_sub_wrapping()` {#jiff_time_sub_wrapping}

```sql
```


## Dates

Dates in `sqlite-jiff` are Gregorian calendar dates, comprised of a year, month, and day, like `2004-06-18`, `1971-04-16`, and `2025-05-26`. 

### Date Constructors

#### `jiff_date()` {#jiff_date}

```sql
```


#### `jiff_date_strptime()` {#jiff_date_strptime}

```sql
```

### Date Components

#### `jiff_date_year()` {#jiff_date_year}

```sql
```

#### `jiff_date_month()` {#jiff_date_month}

```sql
```

#### `jiff_date_day()` {#jiff_date_day}

```sql
```

#### `jiff_date_era()` {#jiff_date_era}

```sql
```


#### `jiff_date_era_year()` {#jiff_date_era_year}

```sql
```


#### `jiff_date_weekday()` {#jiff_date_weekday}

```sql
```

#### `jiff_date_weekday_idx()` {#jiff_date_weekday_idx}

Returns the weekday as an integer, where Monday is `0` and Sunday is `6`.

```sql
select jiff_date_weekday_idx('2024-01-15');
-- 0
select jiff_date_weekday_idx('2024-01-21');
-- 6
```


### Date Utilities

#### `jiff_date_valid()` {#jiff_date_valid}

```sql
```


#### `jiff_date_strftime()` {#jiff_date_strftime}

```sql
```

#### `jiff_date_round()` {#jiff_date_round}

```sql
```

#### `jiff_date_first_of_month()` {#jiff_date_first_of_month}

```sql
```

#### `jiff_date_last_of_month()` {#jiff_date_last_of_month}

```sql
```

#### `jiff_date_days_in_month()` {#jiff_date_days_in_month}

```sql
```

#### `jiff_date_first_of_year()` {#jiff_date_first_of_year}

```sql
```

#### `jiff_date_last_of_year()` {#jiff_date_last_of_year}

```sql
```

#### `jiff_date_days_in_ear()` {#jiff_date_days_in_ear}

```sql
```

#### `jiff_date_in_leap_year()` {#jiff_date_in_leap_year}

```sql
```

#### `jiff_date_tomorrow()` {#jiff_date_tomorrow}

```sql
```

#### `jiff_date_yesterday()` {#jiff_date_yesterday}

```sql
```

#### `jiff_date_nth_weekday_of_month()` {#jiff_date_nth_weekday_of_month}

```sql
```

#### `jiff_date_nth_weekday()` {#jiff_date_nth_weekday}

```sql
```

### Date Arithmetic

#### `jiff_date_add()` {#jiff_date_add}

```sql
```

#### `jiff_date_sub()` {#jiff_date_sub}

```sql
```

#### `jiff_date_add_wrapping()` {#jiff_date_add_wrapping}

```sql
```

#### `jiff_date_sub_wrapping()` {#jiff_date_sub_wrapping}

```sql
```

#### `jiff_date_until()` {#jiff_date_until}

```sql
```

#### `jiff_date_since()` {#jiff_date_since}

```sql
```



#### `jiff_date_series()` {#jiff_date_series}

```sql
```

## DateTime

In `sqlite-jiff`, a datetime is comprised of both a [date](#dates) and a [time](#time), with a year, month, day, hour, minute, second, and optionally sub-second compontent.

Keep in mind, datetimes in `sqlite-jiff` have no concept of timezones or daylight savings. For that, see [zoned](#zoned).

Internally, datetimes are backed by [`jiff::civil::DateTime](https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html).


### DateTime Constructors

#### `jiff_datetime()` {#jiff_datetime}

Construct a datetime from an ISO-8601 string (with a `T` or space separator), or
from separate date and time strings.

```sql
select jiff_datetime('2024-10-31T00:00:00');
-- '2024-10-31T00:00:00'
select jiff_datetime('2024-10-31 00:00:00');
-- '2024-10-31T00:00:00'
select jiff_datetime('2024-10-31', '00:00:00');
-- '2024-10-31T00:00:00'
```

#### `jiff_datetime_strptime()` {#jiff_datetime_strptime}

Parse a datetime from a custom [`strptime`](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html) format string.

```sql
select jiff_datetime_strptime('%F %H:%M', '2024-07-14 21:14');
-- '2024-07-14T21:14:00'
```

### DateTime Utilities

#### `jiff_datetime_start_of_day()` {#jiff_datetime_start_of_day}

```sql
```

#### `jiff_datetime_series()` {#jiff_datetime_series}

A table function that generates a series of datetimes, starting at a given
datetime and stepping by a [span](#span). Runs infinitely, so bound it with a
`LIMIT` or a `datetime < ...` constraint.

```sql
select * from jiff_datetime_series('2023-07-15 16:30:00', '5 hours') limit 5;
/*
┌───────────────────────┐
│ datetime              │
├───────────────────────┤
│ '2023-07-15T16:30:00' │
│ '2023-07-15T21:30:00' │
│ '2023-07-16T02:30:00' │
│ '2023-07-16T07:30:00' │
│ '2023-07-16T12:30:00' │
└───────────────────────┘
*/
```

- [ ] `jiff_datetime_round()`
- [ ] `jiff_datetime_add()` sub etc.
- [ ] `jiff_datetime_tomorrow()`
- [ ] `jiff_datetime_yesterday()`
- [ ] most methods just use the date/time equivalent, as long as they dont require datetime results


## Timestamps

https://docs.rs/jiff/latest/jiff/struct.Timestamp.html

#### `jiff_timestamp()` {#jiff_timestamp}

Construct a timestamp (an instant in UTC time) from an RFC-3339 string. With no
arguments, returns the current time.

```sql
select jiff_timestamp('2024-01-01T23:57:00.123456Z');
-- '2024-01-01T23:57:00.123456Z'
select typeof(jiff_timestamp());
-- 'text'
```

#### `jiff_timestamp_strptime()` {#jiff_timestamp_strptime}

Parse a timestamp from a custom [`strptime`](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html)
format string. The format must include a time zone offset directive (e.g. `%z`).

```sql
select jiff_timestamp_strptime('%Y-%m-%d %H:%M:%S %z', '2024-01-01 00:00:00 +0000');
-- '2024-01-01T00:00:00Z'
```

- [ ] `jiff_timestamp_from_second()`
- [ ] `jiff_timestamp_from_millisecond()`
- [ ] `jiff_timestamp_from_nanosecond()`
- [ ] `jiff_timestamp_from_microsecond()`
- [ ] `jiff_timestamp_round()`
- [ ] `jiff_timestamp_add()` sub etc
- [ ] `jiff_timestamp_series()`
- [ ] `jiff_timestamp_since()`/until etc
- [ ] `jiff_timstamp_strftime` / strptime


### `jiff_timestamp_now()` {#jiff_timestamp_now}

Returns the current timestamp. The result changes on every call, so this
example is illustrative and not executed by docgen.

```
select jiff_timestamp_now();
-- '2026-07-31T19:05:42.240638Z'
```

### `jiff_timestamp_from_milliseconds()` {#jiff_timestamp_from_milliseconds}

Construct a timestamp from a Unix epoch value in milliseconds.

```sql
select jiff_timestamp_from_milliseconds(1704067200000);
-- '2024-01-01T00:00:00Z'
```

#### `jiff_timestamp_from_ms()` {#jiff_timestamp_from_ms}

Short alias for [`jiff_timestamp_from_milliseconds()`](#jiff_timestamp_from_milliseconds).

```sql
select jiff_timestamp_from_ms(0);
-- '1970-01-01T00:00:00Z'
select jiff_timestamp_from_ms(1234);
-- '1970-01-01T00:00:01.234Z'
```


### `jiff_timestamp_as_seconds()` {#jiff_timestamp_as_seconds}

Return the timestamp as a Unix epoch integer. `_as_seconds`, `_as_milliseconds`,
and `_as_microseconds` each have a short alias (`_as_s`, `_as_ms`, `_as_us`).

```sql
select jiff_timestamp_as_seconds('2024-01-01T00:00:00Z');
-- 1704067200
select jiff_timestamp_as_milliseconds('2024-01-01T00:00:00Z');
-- 1704067200000
select jiff_timestamp_as_microseconds('2024-01-01T00:00:00Z');
-- 1704067200000000
```

#### `jiff_timestamp_as_s()` {#jiff_timestamp_as_s}

Short alias for [`jiff_timestamp_as_seconds()`](#jiff_timestamp_as_seconds).

```sql
select jiff_timestamp_as_s('2024-01-01T00:00:00Z');
-- 1704067200
```

#### `jiff_timestamp_as_milliseconds()` {#jiff_timestamp_as_milliseconds}

```sql
select jiff_timestamp_as_milliseconds('2025-06-01T13:59:59.1234Z');
-- 1748786399123
```

#### `jiff_timestamp_as_ms()` {#jiff_timestamp_as_ms}

Short alias for [`jiff_timestamp_as_milliseconds()`](#jiff_timestamp_as_milliseconds).

```sql
select jiff_timestamp_as_ms('2025-06-01T13:59:59.1234Z');
-- 1748786399123
```

#### `jiff_timestamp_as_microseconds()` {#jiff_timestamp_as_microseconds}

```sql
select jiff_timestamp_as_microseconds('2025-06-01T13:59:59.1234Z');
-- 1748786399123400
```

#### `jiff_timestamp_as_us()` {#jiff_timestamp_as_us}

Short alias for [`jiff_timestamp_as_microseconds()`](#jiff_timestamp_as_microseconds).

```sql
select jiff_timestamp_as_us('2025-06-01T13:59:59.1234Z');
-- 1748786399123400
```


## Timezones and Zoned DateTimes {#zoned}

A zoned datetime is a datetime associated with a specific IANA time zone, so it
accounts for offsets and daylight-savings transitions. Internally backed by
[`jiff::Zoned`](https://docs.rs/jiff/latest/jiff/struct.Zoned.html).

#### `jiff_zoned()` {#jiff_zoned}

Construct a zoned datetime from a datetime and a time zone name.

```sql
select jiff_zoned('2024-11-02T01:59:59', 'utc');
-- '2024-11-02T01:59:59+00:00[UTC]'
select jiff_zoned('2024-11-02T01:59:59', 'America/Los_Angeles');
-- '2024-11-02T01:59:59-07:00[America/Los_Angeles]'
```

#### `jiff_zoned_in_tz()` {#jiff_zoned_in_tz}

Convert a zoned datetime into a different time zone, preserving the same instant.

```sql
select jiff_zoned_in_tz('2024-01-01T00:00:00-05:00[America/New_York]', 'America/Los_Angeles');
-- '2023-12-31T21:00:00-08:00[America/Los_Angeles]'
```

#### `jiff_zoned_strptime()` {#jiff_zoned_strptime}

Parse a zoned datetime from a custom [`strptime`](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html) format string.

```sql
select jiff_zoned_strptime('%Y-%m-%dT%H:%M:%S[%Q]', '2024-01-01T00:00:00[America/New_York]');
-- '2024-01-01T00:00:00-05:00[America/New_York]'
```

#### `jiff_timezone_is_available()` {#jiff_timezone_is_available}

Return `1` if the given IANA time zone name is available in the system's time
zone database, `0` otherwise.

```sql
select jiff_timezone_is_available('America/New_York');
-- 1
select jiff_timezone_is_available('Not/AZone');
-- 0
```

#### `jiff_timezone_transitions()` {#jiff_timezone_transitions}

A table function listing the daylight-savings / offset transitions for a time zone.

```sql
select * from jiff_timezone_transitions limit 5;
/*
┌────────────────────────┬────────┬─────┬──────────────┐
│ timestamp              │ offset │ dst │ abbreviation │
├────────────────────────┼────────┼─────┼──────────────┤
│ '2026-11-01T09:00:00Z' │ '-08'  │ 0   │ 'PST'        │
│ '2027-03-14T10:00:00Z' │ '-07'  │ 1   │ 'PDT'        │
│ '2027-11-07T09:00:00Z' │ '-08'  │ 0   │ 'PST'        │
│ '2028-03-12T10:00:00Z' │ '-07'  │ 1   │ 'PDT'        │
│ '2028-11-05T09:00:00Z' │ '-08'  │ 0   │ 'PST'        │
└────────────────────────┴────────┴─────┴──────────────┘
*/
```


## Time Span and Durations {#span}

A "span" refers to a span of time, or a duration of time. A span is comprised of multiple time and calendar units like minutes, seconds, and weeks.


```sql
select jiff_span('5 minutes 10 seconds');
-- '5 minutes, 10 seconds'
select jiff_add('12:31:00', jiff_span('5 minutes 10 seconds'));
-- '12:36:10'
select jiff_until('2025-01-01', '2025-01-12');
-- '11 days'
```

### Span Utilities
Internally, spans are backed by [`jiff::Span`](https://docs.rs/jiff/latest/jiff/struct.Span.html). 

#### `jiff_span()` {#jiff_span}

```sql
select jiff_span('P2m10dT2h30m');
-- '2 months, 10 days, 2 hours, 30 minutes'
```


#### `jiff_span_round()` {#jiff_span_round}

- `largest`
- `smallest`

```sql
select jiff_span_round('17 min 10 sec', 'minute');
-- '17 minutes'
select jiff_span_round('17 min 10 sec', 5, 'minute');
-- '15 minutes'
select jiff_span_round('17 min 10 sec', 2, 'minute');
-- '18 minutes'
```

```sql
select jiff_span_round(
  '10 minutes 5 seconds 589 milliseconds 204 microseconds',
  'largest', 'minute',
  'smallest', 'milliseconds'
);
-- '10 minutes, 5 seconds, 589 milliseconds'
select jiff_span_round(
  '10 minutes 5 seconds 589 milliseconds 204 microseconds',
  'largest', 'seconds',
  'smallest', 'microseconds'
);
-- '605 seconds, 589 milliseconds, 204 microseconds'
select jiff_span_round(
  '10 minutes 5 seconds 589 milliseconds 204 microseconds',
  'largest', 'seconds',
  'smallest', 'microseconds',
  'increment', 5
);
-- '605 seconds, 589 milliseconds, 205 microseconds'
```



```sql
select jiff_span_round(
  '36 minutes',
  'smallest', 'minute',
  'increment', 5,
  'mode', 'expand'
);
-- '40 minutes'
```

#### `jiff_span_total()` {#jiff_span_total}

Return the total length of a span expressed in a single unit, as a floating-point
number. Units larger than days (`week`, `month`, `year`) require a relative
reference date as a third argument.

```sql
select jiff_span_total('2 minutes', 'second');
-- 120.0
select jiff_span_total('45 days', 'month', '2024-01-01');
-- 1.4827586206896552
```

#### `jiff_span_format()` {#jiff_span_format}

Format a span into a human-friendly string. Accepts optional `key`, `value`
option pairs: `spacing`, `designator`, `direction`, and `comma-after-designator`.

```sql
select jiff_span_format('1 hour 42 minutes 23 seconds 999 milliseconds');
-- '1h 42m 23s 999ms'
select jiff_span_format('1 hour 42 minutes', 'designator', 'short');
-- '1hr 42mins'
select jiff_span_format('1 hour 42 minutes ago', 'direction', 'suffix');
-- '1h 42m ago'
```

### Span Arithmetic


#### `jiff_add()` {#jiff_add}

```sql
select jiff_add('2025-01-01', '33 days');
-- '2025-02-03T00:00:00'
select jiff_add('12:00:01', '2 hours 45 seconds 502 milliseconds');
-- '14:00:46.502'
select jiff_add('2025-01-01 12:00:01', '33 days 2 hours 45 seconds 502 milliseconds');
-- '2025-02-03T14:00:46.502'
```

#### `jiff_until()` {#jiff_until}

Return the span from the first datetime-like value *until* the second. Works on
dates, times, and datetimes.

```sql
select jiff_until('2024-01-01', '2024-01-02');
-- '1 day'
select jiff_until('12:00:00', '13:59:59.999');
-- '1 hour, 59 minutes, 59 seconds, 999 milliseconds'
```

#### `jiff_since()` {#jiff_since}

Return the span *since* the second value, relative to the first (the inverse of
[`jiff_until()`](#jiff_until)).

```sql
select jiff_since('2024-01-02', '2024-01-01');
-- '1 day'
select jiff_since('13:59:59.999', '12:00:00');
-- '1 hour, 59 minutes, 59 seconds, 999 milliseconds'
```


## Utilities

#### `jiff_version()` {#jiff_version}

Return the version string of the `sqlite-jiff` extension.

```sql
select jiff_version();
-- 'v0.0.1-alpha.2'
```

#### `jiff_debug()` {#jiff_debug}

Return build/version debug information about the extension. The output
embeds the build's git commit, so this example is illustrative and not
executed by docgen.

```
select jiff_debug();
-- 'Version: v0.0.1-alpha.2
-- Source: e728fd617e35aed4b9475e37fb07209a95bb7180
-- '
```

#### `tzif()` {#tzif}

Parse a [TZif](https://datatracker.ietf.org/doc/html/rfc8536) binary blob (as
found in the system time zone database) into a named time zone. Takes a name and
the raw TZif bytes, and returns a time zone value that can be passed where a
time zone is expected (e.g. [`jiff_zoned()`](#jiff_zoned)).

The blob below is a hand-built TZif for a fixed `+05:45` offset zone; in practice
you would `readfile()` a system zoneinfo file such as
`/usr/share/zoneinfo/America/New_York`.

```sql
select jiff_zoned(
  '2024-07-01T12:00:00',
  tzif('Custom/Plus0545', x'545a696632000000000000000000000000000000000000000000000000000000000000000000000100000004000050dc00004e505400545a696632000000000000000000000000000000000000000000000000000000000000000000000100000004000050dc00004e5054000a3c2b303534353e2d353a34350a')
);
-- '2024-07-01T12:00:00+05:45[Custom/Plus0545]'
```
