use jiff::Timestamp;
use sqlite_loadable::{api, define_scalar_function, prelude::*, Error, Result};

pub fn result_timestamp(context: *mut sqlite3_context, timestamp: Timestamp) -> Result<()> {
    api::result_text(context, timestamp.to_string())
}
pub fn timestamp_from_value(value: &*mut sqlite3_value) -> Result<Timestamp> {
    let input = api::value_text(value)?;
    let t = input.parse();
    match t {
        Ok(timestamp) => Ok(timestamp),
        Err(e) => Err(Error::new_message(e.to_string())),
    }
}

fn jiff_timestamp(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    if values.is_empty() {
        result_timestamp(context, Timestamp::now())?;
        return Ok(());
    }
    match timestamp_from_value(&values[0]) {
        Ok(timestamp) => result_timestamp(context, timestamp)?,
        Err(_) => api::result_null(context),
    }
    Ok(())
}

pub fn jiff_timestamp_from_ms(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let ms = api::value_int64(&values[0]);
    let time = Timestamp::from_millisecond(ms).unwrap();
    api::result_text(context, time.to_string())?;
    Ok(())
}

pub fn jiff_timestamp_strptime(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let format = api::value_blob(&values[0]);
    let input = api::value_blob(&values[1]);
    match jiff::fmt::strtime::parse(format, input) {
        Ok(time) => match time.to_timestamp() {
            Ok(timestamp) => result_timestamp(context, timestamp)?,
            Err(error) => return Err(Error::new_message(error.to_string())),
        },
        Err(_) => {
            // parsing errors return NULL to make it easier to coalesce multiple formats
            api::result_null(context);
        }
    }
    Ok(())
}

pub fn register(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(db, "jiff_timestamp", 0, jiff_timestamp, FunctionFlags::UTF8)?;
    define_scalar_function(db, "jiff_timestamp", 1, jiff_timestamp, FunctionFlags::UTF8)?;

    define_scalar_function(
        db,
        "jiff_timestamp_strptime",
        2,
        jiff_timestamp_strptime,
        FunctionFlags::UTF8,
    )?;
    define_scalar_function(
        db,
        "jiff_timestamp_from_ms",
        1,
        jiff_timestamp_from_ms,
        FunctionFlags::DETERMINISTIC,
    )?;

    Ok(())
}
