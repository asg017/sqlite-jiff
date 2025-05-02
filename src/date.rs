use jiff::{
    civil::{Date, Era},
    fmt::temporal::DateTimeParser,
};
use sqlite_loadable::{api, define_scalar_function, prelude::*, Error, Result};
use std::str::FromStr;

pub(crate) static DEFAULT_DATETIME_PARSER: DateTimeParser = DateTimeParser::new();

pub fn date_from_value(value: &*mut sqlite3_value) -> Result<Date> {
    let input = api::value_text(value)?;
    match Date::from_str(input) {
        Ok(date) => Ok(date),
        Err(e) => Err(Error::new_message(e.to_string())),
    }
}

pub fn result_date(context: *mut sqlite3_context, date: Date) -> Result<()> {
  api::result_text(context, date.to_string())
}

pub fn jiff_date_from_value(value: &*mut sqlite3_value) -> Result<Date> {
    let input = api::value_text(value)?;
    match Date::from_str(input) {
        Ok(date) => Ok(date),
        Err(e) => Err(Error::new_message(e.to_string())),
    }
}

fn jiff_date_weekday(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let date = jiff_date_from_value(&values[0])?;
    let weekday = date.weekday();
    api::result_text(context, match weekday {
        jiff::civil::Weekday::Monday => "Monday",
        jiff::civil::Weekday::Tuesday => "Tuesday",
        jiff::civil::Weekday::Wednesday => "Wednesday",
        jiff::civil::Weekday::Thursday => "Thursday",
        jiff::civil::Weekday::Friday => "Friday",
        jiff::civil::Weekday::Saturday => "Saturday",
        jiff::civil::Weekday::Sunday => "Sunday",
    })?;
    Ok(())
}



pub fn jiff_date_valid(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    // TODO: Should this instead validate that the string is YYYY-MM-DD?
    api::result_bool(
        context,
        DEFAULT_DATETIME_PARSER
            .parse_date(api::value_blob(&values[0]))
            .is_ok(),
    );
    Ok(())
}

pub fn jiff_date_strptime(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let format = api::value_blob(&values[0]);

    // TODO fix null crashes
    if api::value_is_null(&values[1]) {
        api::result_null(context);
        return Ok(());
    }

    let input = api::value_blob(&values[1]);
    match jiff::fmt::strtime::parse(format, input) {
        Ok(time) => match time.to_date() {
            Ok(date) => api::result_text(context, date.to_string())?,
            Err(error) => return Err(Error::new_message(error.to_string())),
        },
        Err(_) => {
            // parsing errors return NULL to make it easier to coalesce multiple formats
            api::result_null(context);
        }
    }
    Ok(())
}
pub fn jiff_date(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    match values.len() {
        1 => match date_from_value(&values[0]) {
            Ok(date) => result_date(context, date)?,
            Err(_) => api::result_null(context),
        },
        3 => {
            let year: i16 = api::value_int64(&values[0])
                .try_into()
                .map_err(|_e| Error::new_message("TODO"))?;
            let month: i8 = api::value_int64(&values[1])
                .try_into()
                .map_err(|_e| Error::new_message("TODO"))?;
            let day: i8 = api::value_int64(&values[2])
                .try_into()
                .map_err(|_e| Error::new_message("TODO"))?;
            result_date(
                context,
                Date::new(year, month, day).map_err(|e| Error::new_message("asdf"))?,
            )?;
        }
        _ => unreachable!(""),
    }

    Ok(())
}

pub fn jiff_date_year(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int(context, date_from_value(&values[0])?.year().into());
    Ok(())
}
pub fn jiff_date_month(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int(context, date_from_value(&values[0])?.month().into());
    Ok(())
}
pub fn jiff_date_day(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int(context, date_from_value(&values[0])?.day().into());
    Ok(())
}
pub fn jiff_date_era(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (_, era) = date_from_value(&values[0])?.era_year();
    api::result_text(
        context,
        match era {
            Era::BCE => "BCE",
            Era::CE => "CE",
        },
    )?;
    Ok(())
}
pub fn jiff_date_era_year(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let (year, _) = date_from_value(&values[0])?.era_year();
    api::result_int(context, year.into());
    Ok(())
}

pub fn jiff_date_strftime(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let input = api::value_blob(&values[0]);
    let format = api::value_blob(&values[1]);
    let x = DEFAULT_DATETIME_PARSER.parse_date(input).unwrap();
    match jiff::fmt::strtime::format(format, x) {
        Ok(time) => {
            api::result_text(context, time.to_string())?;
        }
        Err(_) => {
            // parsing errors return NULL to make it easier to coalesce multiple formats
            api::result_null(context);
        }
    }
    // TODO: panics? https://docs.rs/jiff/latest/jiff/civil/struct.Date.html#method.strftime
    //api::result_text(context, date.strftime(format).to_string())?;
    Ok(())
}

pub fn register(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::UTF8;
    define_scalar_function(db, "jiff_date", 1, jiff_date, flags)?;
    define_scalar_function(db, "jiff_date", 3, jiff_date, flags)?;

    define_scalar_function(db, "jiff_date_strptime", 2, jiff_date_strptime, flags)?;
    define_scalar_function(db, "jiff_date_strftime", 2, jiff_date_strftime, flags)?;
    define_scalar_function(
        db,
        "jiff_date_valid",
        1,
        jiff_date_valid,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_date_year",
        1,
        jiff_date_year,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_date_month",
        1,
        jiff_date_month,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_date_day",
        1,
        jiff_date_day,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_date_era",
        1,
        jiff_date_era,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_date_era_year",
        1,
        jiff_date_era_year,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_date_weekday",
        1,
        jiff_date_weekday,
        FunctionFlags::DETERMINISTIC,
    )?;

    Ok(())
}
