use jiff::civil::DateTime;
use sqlite_loadable::{
    api, define_scalar_function, define_table_function, prelude::*, Error, Result,
};
use std::str::FromStr;

use crate::jiff_datetime_series::DatetimeSeriesTable;
use crate::{date::date_from_value, time::time_from_value};

pub fn datetime_from_value(value: &*mut sqlite3_value) -> Result<DateTime> {
    let input = api::value_text(value)?;
    match DateTime::from_str(input) {
        Ok(datetime) => Ok(datetime),
        Err(e) => Err(Error::new_message(e.to_string())),
    }
}

pub fn result_datetime(context: *mut sqlite3_context, datetime: DateTime) -> Result<()> {
    api::result_text(context, datetime.to_string())
}

pub fn jiff_datetime_strptime(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let format = api::value_blob(&values[0]);
    let input = api::value_blob(&values[1]);
    match jiff::fmt::strtime::parse(format, input) {
        Ok(time) => match time.to_datetime() {
            Ok(datetime) => result_datetime(context, datetime)?,
            Err(error) => return Err(Error::new_message(error.to_string())),
        },
        Err(_) => {
            // parsing errors return NULL to make it easier to coalesce multiple formats
            api::result_null(context);
        }
    }
    Ok(())
}

pub fn jiff_datetime(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    match values.len() {
        1 => match datetime_from_value(&values[0]) {
            Ok(datetime) => result_datetime(context, datetime)?,
            Err(_) => api::result_null(context),
        },
        2 => match (date_from_value(&values[0]), time_from_value(&values[1])) {
            (Ok(date), Ok(time)) => {
                result_datetime(context, DateTime::from_parts(date, time))?;
            }
            (Err(_err), Ok(_)) => todo!(),
            (Ok(_), Err(_err)) => todo!(),
            (Err(_e1), Err(_e2)) => todo!(),
        },
        _ => todo!(),
    }

    Ok(())
}
pub fn register(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(db, "jiff_datetime", 1, jiff_datetime, FunctionFlags::UTF8)?;
    define_scalar_function(db, "jiff_datetime", 2, jiff_datetime, FunctionFlags::UTF8)?;
    define_scalar_function(
        db,
        "jiff_datetime_strptime",
        2,
        jiff_datetime_strptime,
        FunctionFlags::UTF8,
    )?;
    define_table_function::<DatetimeSeriesTable>(db, "jiff_datetime_series", None)?;
    Ok(())
}
