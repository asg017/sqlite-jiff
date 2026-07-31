.load dist/debug/jiff0

-- ===========================================================================
-- Shared setup (was tests/snaps/_init.sql). Each `solite test` file runs
-- against its own fresh in-memory database, so setup lives inline here.
-- ===========================================================================

create temp table units as
  select value as unit
  from json_each('[
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
    "week",
    "month",
    "year"
  ]');

create temp table spacings as
  with spacings(spacing) as (
    values ('between-units-and-designators'), ('between-units'), ('none')
  )
  select * from spacings;

create temp table designators as
  with designators(designator) as (
    values ('verbose'), ('short'), ('compact')
  )
  select * from designators;

create temp table directions as
  with directions(direction) as (
    values ('auto'), ('sign'), ('force-sign'), ('suffix')
  )
  select * from directions;


-- ===========================================================================
-- meta
-- ===========================================================================

select regex_replace('(v)(.*)', jiff_version(), '${1}REDACTED'); -- 'vREDACTED'


-- ===========================================================================
-- date
-- ===========================================================================

select jiff_date('2024-01-01');            -- '2024-01-01'
select jiff_date(2024, 10, 12);            -- '2024-10-12'
select jiff_date_day('2024-02-01');        -- 1
select jiff_date_day('2024-02-29');        -- 29
select jiff_date_month('2024-02-01');      -- 2

select jiff_date_era('2024-01-01');        -- 'CE'
select jiff_date_era('0000-01-01');        -- 'BCE'
select jiff_date_era('-000001-01-01');     -- 'BCE'
select jiff_date_era('-002001-01-01');     -- 'BCE'

select jiff_date_era_year('2024-01-01');   -- 2024
select jiff_date_era_year('0000-01-01');   -- 1
select jiff_date_era_year('0001-01-01');   -- 1
select jiff_date_era_year('-000001-01-01');-- 2
select jiff_date_era_year('-002001-01-01');-- 2002

select jiff_date_year('2024-01-13');            -- 2024
select jiff_date_year('2024-01-13 12:00:00');   -- 2024
select jiff_date_year('INVALID');               -- error: failed to parse year in date "INVALID": failed to parse "INVA" as year (a four digit integer): invalid digit, expected 0-9 but got I
select jiff_date_year('9999-12-31');            -- 9999
select jiff_date_year(NULL);                    -- error: failed to parse year in date "": expected four digit year (or leading sign for six digit year), but found end of input

select jiff_date_strptime('%m/%d/%y', '7/14/24');               -- '2024-07-14'
select jiff_date_strptime('%Y-%m-%d', '2024-01-13');            -- '2024-01-13'
select jiff_date_strptime('%Y-%m-%d %H:%M:%S', '2024-01-13 12:00:00'); -- '2024-01-13'
select jiff_date_strptime('invalid', '2024-01-13');            -- NULL
select jiff_date_strptime('%Y', '2024-01-13');                 -- NULL
select jiff_date_strptime('%Y', '2024');                       -- error: a month/day, day-of-year or week date must be present to create a date, but none were found

select value, coalesce(
    jiff_date_strptime('%m/%d/%y', value),
    jiff_date_strptime('%m/%d/%Y', value),
    jiff_date_strptime('%Y-%m-%d', value)
  ) as result
from json_each('["3/14/24","3/14/2024","2024-03-14"]'); -- @snap date_strptime_coalesce

select jiff_date_strftime('2024-01-13', '%Y-%m-%d is a %A'); -- '2024-01-13 is a Saturday'
select jiff_date_strftime('2024-01-13', '%Y-%m-%d at %H');   -- NULL

select value, jiff_date_valid(value)
from json_each('["2024-01-02","2024-01-13 12:00:00","invalid"]'); -- @snap date_valid

select value, jiff_date_weekday(value)
from json_each('[
  "2024-01-13","2024-01-14","2024-01-15","2024-01-16",
  "2024-01-17","2024-01-18","2024-01-19"
]'); -- @snap date_weekday

-- new: jiff_date_weekday_idx (Monday=0 .. Sunday=6)
select value, jiff_date_weekday_idx(value)
from json_each('[
  "2024-01-13","2024-01-14","2024-01-15","2024-01-16",
  "2024-01-17","2024-01-18","2024-01-19"
]'); -- @snap date_weekday_idx


-- ===========================================================================
-- datetime
-- ===========================================================================

select jiff_datetime('2024-10-31T00:00:00');       -- '2024-10-31T00:00:00'
select jiff_datetime('2024-10-31 00:00:00');       -- '2024-10-31T00:00:00'
select jiff_datetime('2024-10-31', '00:00:00');    -- '2024-10-31T00:00:00'

select jiff_datetime_strptime('%F %H:%M', '2024-07-14 21:14'); -- '2024-07-14T21:14:00'

select * from jiff_datetime_series('2023-07-15 16:30:00', '5 hours') limit 10;  -- @snap datetime_series_pos
select * from jiff_datetime_series('2023-07-15 16:30:00', '-5 hours') limit 10; -- @snap datetime_series_neg
select * from jiff_datetime_series('2023-07-15 16:30:00', '0 seconds') limit 10; -- error: jiff_datetime_series span cannot be zero


-- ===========================================================================
-- date series (new)
-- ===========================================================================

select * from jiff_date_series('2024-01-01', 'P1D') limit 5; -- @snap date_series


-- ===========================================================================
-- time
-- ===========================================================================

select jiff_time('12:00:00');           -- '12:00:00'
select jiff_time('23:59:59.999999');    -- '23:59:59.999999'
select jiff_time('1:30 pm');            -- NULL
select jiff_time(23, 58, 59);           -- '23:58:59'
select jiff_time(23, 58, 59, 999999);   -- '23:58:59.000999999'

select jiff_time_round('12:59:00', 'hour'); -- '13:00:00'
select jiff_time_round('12:29:00', 'hour'); -- '12:00:00'

with test_cases(time, mode, smallest, increment) as (
  values
    ('12:59:00', 'floor', 'hour', 1),
    ('12:01:00', 'ceil', 'minute', 20)
)
select test_cases.*, jiff_time_round(
    time, 'mode', mode, 'smallest', smallest, 'increment', increment
  ) as result
from test_cases; -- @snap time_round_opts

with test_cases(time) as (
  values ('12:00:00'), ('23:59:59.999999'), ('01:30')
)
select time, jiff_time_hour(time) from test_cases; -- @snap time_hour


-- ===========================================================================
-- timestamp
-- ===========================================================================

select typeof(jiff_timestamp());                        -- 'text'
select jiff_timestamp('2024-01-01T23:57:00.123456Z');   -- '2024-01-01T23:57:00.123456Z'
select typeof(jiff_timestamp_now());                    -- 'text'

-- from milliseconds (canonical + legacy alias)
select jiff_timestamp_from_milliseconds(1704067200000); -- '2024-01-01T00:00:00Z'
select jiff_timestamp_from_ms(1704067200000);           -- '2024-01-01T00:00:00Z'

-- as_* accessors + short aliases
select jiff_timestamp_as_seconds('2024-01-01T00:00:00Z');      -- 1704067200
select jiff_timestamp_as_s('2024-01-01T00:00:00Z');            -- 1704067200
select jiff_timestamp_as_milliseconds('2024-01-01T00:00:00Z'); -- 1704067200000
select jiff_timestamp_as_ms('2024-01-01T00:00:00Z');           -- 1704067200000
select jiff_timestamp_as_microseconds('2024-01-01T00:00:00Z'); -- 1704067200000000
select jiff_timestamp_as_us('2024-01-01T00:00:00Z');           -- 1704067200000000


-- ===========================================================================
-- span: until / since / add
-- ===========================================================================

select jiff_until('2024-01-01', '2024-01-02');                          -- '1 day'
select jiff_until('12:00:00', '13:59:59.999');                          -- '1 hour, 59 minutes, 59 seconds, 999 milliseconds'
select jiff_until('2024-01-01 12:00:00', '2024-01-02 13:59:59.999');    -- '1 day, 1 hour, 59 minutes, 59 seconds, 999 milliseconds'

with testcases(a, b) as (
  values
    ('2024-01-01', '2024-01-02'),
    ('12:00:00', '13:59:59.999'),
    ('2024-01-01 12:00:00', '2024-01-02 13:59:59.999')
)
select a, b, jiff_until(a, b), jiff_since(a, b) from testcases; -- @snap until_since

select jiff_since('2024-01-02', '2024-01-01');                          -- '1 day'
select jiff_since('13:59:59.999', '12:00:00');                          -- '1 hour, 59 minutes, 59 seconds, 999 milliseconds'
select jiff_since('2024-01-02 13:59:59.999', '2024-01-01 12:00:00');    -- '1 day, 1 hour, 59 minutes, 59 seconds, 999 milliseconds'

with testcases(date, span) as (
  values
    ('2024-01-01', '1 day'),
    ('2024-01-01', '1 hour'),
    ('2024-01-01', '1 minute'),
    ('2024-01-01', '1 second'),
    ('2024-01-01', '1 millisecond'),
    ('2024-01-01', '1 microsecond'),
    ('2024-01-01', '1 nanosecond')
)
select date, span, jiff_add(date, span) as result from testcases; -- @snap add


-- ===========================================================================
-- span: jiff_span (new), round, total, format
-- ===========================================================================

select jiff_span('1 hour 30 minutes'); -- '1 hour, 30 minutes'
select jiff_span('PT1H30M');           -- '1 hour, 30 minutes'

select jiff_span_round('1 second 400 milliseconds');            -- '1 second, 400 milliseconds'
select jiff_span_round('1 hour 30 minutes', 'hour');            -- '2 hours'
select jiff_span_round('90 minutes', 5, 'minute');             -- '90 minutes'
select jiff_span_round('1 hour 30 minutes', 'smallest', 'hour', 'mode', 'half-expand'); -- '2 hours'
-- relative option (needed to round across variable-length units)
select jiff_span_round('45 days', 'smallest', 'month', 'relative', '2024-01-01'); -- '1 month'

with testcases(span) as (values ('2 minutes'), ('1 nanosecond'))
select span, unit, jiff_span_total(span, unit)
from testcases
join units
where unit not in ('year', 'week', 'month')
order by span, units.rowid; -- @snap span_total

select jiff_span_total('2 minutes', 'week'); -- error: using unit 'week' in a span or configuration requires that a relative reference time be given, but none was provided
-- 3-arg total with relative date
select jiff_span_total('45 days', 'month', '2024-01-01'); -- 1.48275862068966

select jiff_span_format('2 minutes'); -- '2m'

with testcases(value) as (values ('1 hour 42 minutes 23 seconds 999 milliseconds'))
select value, spacing, jiff_span_format(value, 'spacing', spacing) as result
from testcases, spacings; -- @snap span_format_spacings
select jiff_span_format('1 minute', 'spacing', 'invalid'); -- error: Unknown spacing 'invalid'

with testcases(value) as (values ('1 hour 42 minutes 23 seconds 999 milliseconds'))
select value, designator, jiff_span_format(value, 'designator', designator) as result
from testcases, designators; -- @snap span_format_designators
select jiff_span_format('1 minute', 'designator', 'invalid'); -- error: Unknown designator invalid

with testcases(value) as (values ('1 hour 42 minutes'), ('1 hour 42 minutes ago'))
select value, direction, jiff_span_format(value, 'direction', direction) as result
from testcases, directions; -- @snap span_format_directions
select jiff_span_format('1 minute', 'direction', 'invalid'); -- error: Unknown direction invalid


-- ===========================================================================
-- zoned
-- ===========================================================================

select jiff_zoned('2024-11-02T01:59:59', 'utc');                    -- '2024-11-02T01:59:59+00:00[UTC]'
select jiff_zoned('2024-11-02T01:59:59', 'America/Los_Angeles');    -- '2024-11-02T01:59:59-07:00[America/Los_Angeles]'

with test_cases(value) as (
  VALUES
    ('2024-11-02T01:59:59[America/Los_Angeles]'),
    ('2024-11-02T02:00:01[America/New_York]'),
    ('2024-11-02T02:00:01[+00:40:00]')
)
select value from test_cases order by value collate jiff_zoned_cmp; -- @snap zoned_cmp

-- tzif: parse a raw TZif blob into a custom-named zone, then use it as a
-- timezone argument. This is a hand-built, self-contained TZif v2 file for a
-- fixed +05:45 offset zone (no filesystem / system tzdb dependency), so it
-- resolves to +05:45 year-round regardless of the process TZ.
-- Regenerate the blob with: python3 tests/fixtures/gen_tzif.py
select jiff_zoned(
  '2024-07-01T12:00:00',
  tzif('Custom/Plus0545', x'545a696632000000000000000000000000000000000000000000000000000000000000000000000100000004000050dc00004e505400545a696632000000000000000000000000000000000000000000000000000000000000000000000100000004000050dc00004e5054000a3c2b303534353e2d353a34350a')
); -- '2024-07-01T12:00:00+05:45[Custom/Plus0545]'
select jiff_zoned(
  '2024-01-01T12:00:00',
  tzif('Custom/Plus0545', x'545a696632000000000000000000000000000000000000000000000000000000000000000000000100000004000050dc00004e505400545a696632000000000000000000000000000000000000000000000000000000000000000000000100000004000050dc00004e5054000a3c2b303534353e2d353a34350a')
); -- '2024-01-01T12:00:00+05:45[Custom/Plus0545]'


-- ===========================================================================
-- timezone transitions
-- ===========================================================================

select * from jiff_timezone_transitions limit 10; -- @snap timezone_transitions
