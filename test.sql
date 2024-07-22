.load ../target/debug/libsqlite_jiff.dylib sqlite3_jiff_init

.mode box
.header on

select jiff_duration(
  '2024-11-02T01:59:59[America/Los_Angeles]',
  '2024-11-02T02:00:01[America/New_York]',
  'minutes'
) as result;

select jiff_duration(
  '2024-11-03T01:59:59[America/Los_Angeles]',
  '2024-11-03T02:00:01[America/New_York]',
  'minutes'
) as result;
