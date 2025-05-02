.load dist/debug/jiff0

create temp table units as 
  select 
    value as unit 
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
    "year",
  ]');

-- #region meta
select regex_replace('(v)(.*)', jiff_version(), '${1}REDACTED');

select 
  regex_replace(
    '(?<key>[^:]:)(.*)', 
    line, 
    '$key REDACTED'
  ) as line_redacted
from lines(jiff_debug());

-- #endregion

-- #region jiff_date_strptime


select jiff_date_strptime('%m/%d/%y', '7/14/24');
select jiff_date_strptime('%Y-%m-%d', '2024-01-13');

-- since this is jiff_date*, only YYYY-MM-DD is outputed
select jiff_date_strptime('%Y-%m-%d %H:%M:%S', '2024-01-13 12:00:00');

-- doesnt match the format
select jiff_date_strptime('invalid', '2024-01-13');

-- "must consume entire input"
select jiff_date_strptime('%Y', '2024-01-13');

-- must be an entire date
select jiff_date_strptime('%Y', '2024');


select 
  value,
  coalesce(
    jiff_date_strptime('%m/%d/%y', value),
    jiff_date_strptime('%m/%d/%Y', value),
    jiff_date_strptime('%Y-%m-%d', value)
  ) as result
from json_each('[
  "3/14/24",
  "3/14/2024",
  "2024-03-14",
]');
-- #endregion

-- #region jiff_date_strftime
select jiff_date_strftime('2024-01-13', '%Y-%m-%d is a %A');


select jiff_date_strftime('2024-01-13', '%Y-%m-%d at %H');
-- #endregion

-- #region jiff_date_valid
select 
  value,
  jiff_date_valid(value)
from json_each('[
  "2024-01-02",
  "2024-01-13 12:00:00",
  "invalid"
]');
-- #endregion


-- #region jiff_zoned_cmp
with test_cases(value) as (
  VALUES
    ('2024-11-02T01:59:59[America/Los_Angeles]'),
    ('2024-11-02T02:00:01[America/New_York]'),
    ('2024-11-02T02:00:01[+00:40:00]')
)
select value
from test_cases
order by value collate jiff_zoned_cmp;
-- #endregion

-- #region jiff_zoned

select jiff_zoned('2024-11-02T01:59:59', 'utc');

select jiff_zoned('2024-11-02T01:59:59', 'local');

select jiff_zoned('2024-11-02T01:59:59', 'system');

select jiff_zoned('2024-11-02T01:59:59', 'America/Los_Angeles');
-- #endregion



-- #region jiff_date

-- #region jiff_date_year
select jiff_date_year('2024-01-13');
select jiff_date_year('2024-01-13 12:00:00');
select jiff_date_year('INVALID');
select jiff_date_year('9999-12-31');
-- TODO
--select jiff_date_year('-0000-01-01');
--select jiff_date_year('-9999-01-01');
--select jiff_date_year('-0000-01-01');
--select jiff_date_year('-99999-01-01');
select jiff_date_year(NULL);
-- #endregion

-- #endregion


-- #region jiff_time

select jiff_time('12:00:00');
select jiff_time('23:59:59.999999');
select jiff_time('1:30 pm');

select jiff_time(23, 58, 59);
select jiff_time(23, 58, 59, 999999);


-- #region round
select jiff_time_round('12:59:00', 'hour');
select jiff_time_round('12:29:00', 'hour');

with test_cases(time, mode, smallest, increment) as (
  values
    ('12:59:00', 'floor', 'hour', 1),
    ('12:01:00', 'ceil', 'minute', 20)
)
select 
  test_cases.*,
  jiff_time_round(
    time,
    'mode', mode,
    'smallest', smallest,
    'increment', increment
) as result
from test_cases;
-- #endregion
-- #endregion

-- #region jiff_datetime_series
select * 
from jiff_datetime_series('2023-07-15 16:30:00', '5 hours')
limit 10;

select * 
from jiff_datetime_series('2023-07-15 16:30:00', '-5 hours')
limit 10;

select * 
from jiff_datetime_series('2023-07-15 16:30:00', '0 seconds')
limit 10;
-- #endregion


-- #region jiff_date

--  #region date 
select jiff_date('2024-01-01');
--  #endregion

--  #region date_day 
select jiff_date_day('2024-02-01');
--  #endregion

--  #region date_era 
select jiff_date_era('2024-01-01');
select jiff_date_era('0000-01-01');
select jiff_date_era('-000001-01-01');
select jiff_date_era('-002001-01-01');
--  #endregion

--  #region date_era_year 
select jiff_date_era_year('2024-01-01');
select jiff_date_era_year('0000-01-01');
select jiff_date_era_year('0001-01-01');
select jiff_date_era_year('-000001-01-01');
select jiff_date_era_year('-002001-01-01');
--  #endregion

--  #region date_month 
select jiff_date_month('2024-02-01');
--  #endregion
-- #endregion


-- #region jiff_datetime

--  #region datetime
select jiff_datetime('2024-10-31T00:00:00');
select jiff_datetime('2024-10-31 00:00:00');
select jiff_datetime('2024-10-31', '00:00:00');
--   #endregion

--  #region datetime_strptime
select jiff_datetime_strptime(
  '%F %H:%M', 
  '2024-07-14 21:14'
);
--   #endregion

-- #endregion

--select lol();


-- #region jiff_since

select jiff_since('2024-01-02', '2024-01-01');
select jiff_since('13:59:59.999', '12:00:00');
select jiff_since('2024-01-02 13:59:59.999', '2024-01-01 12:00:00');

-- #endregion

-- #region jiff_until

select jiff_until('2024-01-01', '2024-01-02');
select jiff_until('12:00:00', '13:59:59.999');
select jiff_until('2024-01-01 12:00:00', '2024-01-02 13:59:59.999');

with testcases(a, b) as (
  values
    ('2024-01-01', '2024-01-02'),
    ('12:00:00', '13:59:59.999'),
    ('2024-01-01 12:00:00', '2024-01-02 13:59:59.999')
)
select 
  a,
  b,
  jiff_until(a, b),
  jiff_since(a, b)
from testcases;  


-- #endregion

-- #region jiff_timestamp

--  #region timestamp
select typeof(jiff_timestamp());

select jiff_timestamp('2024-01-01T23:57:00.123456Z');
--  #endregion

--  #region timestamp_from_ms
select  jiff_timestamp_from_ms(1704067200000);
--  #endreion

-- #endregion

-- #region jiff_span

--   #region total
with testcases(span) as (
  values
  ('2 minutes'),
  ('1 nanosecond')
)
select 
  span,
  unit,
  jiff_span_total(span, unit)
from testcases
join units
where unit not in ('year', 'week', 'month')
order by span, units.rowid;

select jiff_span_total('2 minutes', 'year');
select jiff_span_total('2 minutes', 'week');
select jiff_span_total('2 minutes', 'month');
--   #endregion

-- #endregion


-- #region jiff_timezone_transitions
select * 
from jiff_timezone_transitions
limit 10;
-- #endregion



-- #region jiff_add
--select jiff_add('2024-01-01', '1 day');
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
select 
  date,
  span,
  jiff_add(date, span) as result
from testcases;
-- #endregion