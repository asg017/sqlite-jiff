use jiff::{civil::DateTime, fmt::temporal::DateTimeParser, tz::TimeZone, Timestamp, Zoned};
use sqlite_loadable::{
    api::{self, ValueType}, define_collation, define_scalar_function, define_table_function, prelude::*, Error, Result
};
use std::{ffi::CStr, str::FromStr};

use crate::{date::jiff_date_from_value, time::jiff_time_from_value, timezone_transitions::TimezoneTransitionsTable};

static PARSER: DateTimeParser = DateTimeParser::new();

pub fn jiff_zoned_from_value(value: &*mut sqlite3_value) -> Result<Zoned> {
    let input = api::value_text(value)?;
    Zoned::from_str(input).map_err(|e| Error::new_message(e.to_string()))
}

pub fn result_zoned(context: *mut sqlite3_context, zoned: Zoned) -> Result<()> {
  api::result_text(context, zoned.to_string())
}

pub fn jiff_zoned_strptime(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let format = api::value_blob(&values[0]);
    let input = api::value_blob(&values[1]);
    match jiff::fmt::strtime::parse(format, input) {
        Ok(time) => match time.to_zoned() {
            Ok(zoned) => result_zoned(context, zoned)?,
            Err(error) => return Err(Error::new_message(error.to_string())),
        },
        Err(_) => {
            // parsing errors return NULL to make it easier to coalesce multiple formats
            api::result_null(context);
        }
    }
    Ok(())
}


static TIMEZONE_POINTER: &CStr = c"timezone";

fn jiff_timezone_is_available(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
  let name = api::value_text(&values[0])?;
  api::result_bool(context, TimeZone::get(name).is_ok());
  Ok(())
}
fn tzif(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let name = api::value_text(&values[0])?;
    let tz = api::value_blob(&values[1]);
    let tz = TimeZone::tzif(name, tz).map_err(|e| Error::new_message(e.to_string()))?;
    api::result_pointer(context, TIMEZONE_POINTER.to_bytes(), tz);
    Ok(())
}

fn timezone_from_value(value: &*mut sqlite3_value) -> Result<TimeZone> {
    match api::value_type(value) {
        ValueType::Text => match api::value_text(value)?.to_lowercase().as_str() {
            "utc" | "z" => Ok(TimeZone::UTC),
            "system" | "local" => Ok(TimeZone::system()),
            zone => Ok(TimeZone::get(zone).map_err(|e| Error::new_message(format!("Could not resolve timezone '{zone}': {e}")))?),
        },
        /*ValueType::Blob => {
          Ok(TimeZone::tzif("", api::value_blob(value)).map_err(|e| Error::new_message(e.to_string()))?)
        }*/
        ValueType::Null => match unsafe { api::value_pointer::<TimeZone>(value, TIMEZONE_POINTER.to_bytes()) } {
            Some(tz) => {
                let x = unsafe { (*tz).clone() };
                Ok(x)
            }
            None => Err(Error::new_message("no timezone found")),
        },
        _ => todo!(),
    }
}
fn jiff_zoned(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    match values.len() {
        // with 2 args, either:
        //  1. (datetime, tz)
        //  2. (timestamp, tz)
        2 => {
            let zone = timezone_from_value(&values[1])?;
            if let Ok(datetime) = jiff::civil::DateTime::from_str(api::value_text(&values[0])?) {
                match datetime.to_zoned(zone) {
                    Ok(zoned) => api::result_text(context, zoned.to_string())?,
                    Err(error) => return Err(Error::new_message(error.to_string())),
                }
            } else {
                let ts = Timestamp::from_str(api::value_text(&values[0])?).unwrap();
                api::result_text(context, ts.to_zoned(zone).to_string()).unwrap();
            }
        }
        // (date, time, tz)
        3 => {
            let date = jiff_date_from_value(&values[0])?;
            let time = jiff_time_from_value(&values[1])?;
            let datetime = DateTime::from_parts(date, time);
            let tz = timezone_from_value(&values[2])?;
            match datetime.to_zoned(tz) {
              Ok(zoned) => result_zoned(context, zoned)?,
              Err(error) => return Err(Error::new_message(error.to_string())),
            }
        }
        _ => todo!(),
    }
    Ok(())
}

fn jiff_zoned_in_tz(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let zone = jiff_zoned_from_value(&values[0])?;
    let tz = api::value_text(&values[1])?;
    match zone.in_tz(tz) {
        Ok(zoned) => api::result_text(context, zoned.to_string())?,
        Err(error) => return Err(Error::new_message(error.to_string())),
    }
    Ok(())
}

fn jiff_zoned_cmp(a: &[u8], b: &[u8]) -> i32 {
  match (PARSER.parse_zoned(a), PARSER.parse_zoned(b)) {
      (Ok(a), Ok(b)) => match a.cmp(&b) {
          std::cmp::Ordering::Less => -1,
          std::cmp::Ordering::Equal => 0,
          std::cmp::Ordering::Greater => 1,
      },
      (Ok(_), Err(_)) => -1,
      (Err(_), Ok(_)) => 1,
      (Err(_), Err(_)) => 1,
  }
}

pub fn register(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(
        db,
        "jiff_zoned",
        2,
        jiff_zoned,
        FunctionFlags::DETERMINISTIC,
    )?;

    define_scalar_function(
        db,
        "jiff_zoned",
        3,
        jiff_zoned,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_zoned_in_tz",
        2,
        jiff_zoned_in_tz,
        FunctionFlags::DETERMINISTIC,
    )?;

    define_scalar_function(
      db,
      "jiff_zoned_strptime",
      2,
      jiff_zoned_strptime,
      FunctionFlags::UTF8,
  )?;
    define_scalar_function(
      db,
      "jiff_timezone_is_available",
      1,
      jiff_timezone_is_available,
      FunctionFlags::UTF8,
  )?;

    define_scalar_function(db, "tzif", 2, tzif, FunctionFlags::DETERMINISTIC)?;
    define_collation(db, "jiff_zoned_cmp", jiff_zoned_cmp)?;
    define_table_function::<TimezoneTransitionsTable>(db, "jiff_timezone_transitions", None)?;

    Ok(())
}
