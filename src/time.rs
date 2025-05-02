use jiff::{civil::{Time, TimeRound}, RoundMode};
use sqlite_loadable::{api, define_scalar_function, prelude::*, Error, Result};
use std::str::FromStr;

use crate::span::unit_from_value;

pub fn result_time(context: *mut sqlite3_context, time: Time) -> Result<()> {
    api::result_text(context, time.to_string())
}

pub fn time_from_value(value: &*mut sqlite3_value) -> Result<Time> {
    let input = api::value_text(value)?;
    match Time::from_str(input) {
        Ok(time) => Ok(time),
        Err(e) => {
          if input == "midnight" {
            Ok(Time::midnight())
          } else {
            Err(Error::new_message(e.to_string()))
          }
        },
    }
}

fn jiff_time(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
  match values.len() {
    1 => match time_from_value(&values[0]) {
      Ok(time) => result_time(context, time)?,
      Err(_) => api::result_null(context),
    }
    3|4 => {
      let hour = api::value_int(&values[0]).try_into().map_err(|_| Error::new_message("Invalid hour value"))?;
      let minute = api::value_int(&values[1]).try_into().map_err(|_| Error::new_message("Invalid minute value"))?;
      let second = api::value_int(&values[2]).try_into().map_err(|_| Error::new_message("Invalid second value"))?;
      // TODO check if i64
      let subsec_nanosecond = values.get(3).map(|v| api::value_int(v)).unwrap_or(0);
      match Time::new(hour, minute, second, subsec_nanosecond) {
        Ok(time) => result_time(context, time)?,
        Err(e) => return Err(Error::new_message(e.to_string())),
      }
    }
    _ => todo!(),
  }
  Ok(())
}


fn roundmode_from_text(text: &str) -> Option<RoundMode> {
  // Ceil, Floor, Expand, Trunc, HalfCeil, HalfFloor, HalfExpand, HalfTrunc, HalfEven,
    match text {
      "ceil" => Some(RoundMode::Ceil),
      "floor" => Some(RoundMode::Floor),
      "expand" => Some(RoundMode::Expand),
      "trunc" => Some(RoundMode::Trunc),
      "half-ceil" => Some(RoundMode::HalfCeil),
      "half-floor" => Some(RoundMode::HalfFloor),
      "half-expand" => Some(RoundMode::HalfExpand),
      "half-trunc" => Some(RoundMode::HalfTrunc),
      "half-even" => Some(RoundMode::HalfEven),
      _ => None
    }
}
pub fn jiff_time_round(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let time = time_from_value(&values[0])?;
    if values.len()  == 2 {
      let unit = unit_from_value(&values[1])?;
      result_time(context, time.round(unit).map_err(|e| Error::new_message(e.to_string()))?,)?;
      return Ok(());
    }
    let mut round = TimeRound::new();
    for pair in values[1..].iter().as_slice().chunks(2) {
      let k = pair[0];
      let v = pair[1];
      let key = api::value_text(&k)?;
      round = match key {
          "smallest" => round.smallest(unit_from_value(&v)?),
          "increment" => round.increment(api::value_int64(&v)),
          "mode" => {
            match roundmode_from_text(api::value_text(&v)?) {
                Some(mode) => round.mode(mode),
                None => return Err(Error::new_message(format!(
                    "Unknown round mode: '{}'", api::value_text(&v)?
                ))),
            }
          }
          k => {
              return Err(Error::new_message(format!(
                  "Unknown key for jiff_span_round: '{k}'"
              )))
          }
      }
  }
    result_time(context, time.round(round).map_err(|e| Error::new_message(e.to_string()))?)?;
    Ok(())
}

pub fn jiff_time_strptime(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let format = api::value_blob(&values[0]);
    let input = api::value_blob(&values[1]);
    match jiff::fmt::strtime::parse(format, input) {
        Ok(time) => match time.to_time() {
            Ok(time) => api::result_text(context, time.to_string())?,
            Err(error) => return Err(Error::new_message(error.to_string())),
        },
        Err(_) => {
            // parsing errors return NULL to make it easier to coalesce multiple formats
            api::result_null(context);
        }
    }
    Ok(())
}

pub fn jiff_time_hour(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int(context, time_from_value(&values[0])?.hour().into());
    Ok(())
}

pub fn jiff_time_from_value(value: &*mut sqlite3_value) -> Result<Time> {
    let input = api::value_text(value)?;
    match Time::from_str(input) {
        Ok(time) => Ok(time),
        Err(e) => Err(Error::new_message(e.to_string())),
    }
}

pub fn register(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(db, "jiff_time", 1, jiff_time, FunctionFlags::DETERMINISTIC)?;
    define_scalar_function(db, "jiff_time", 3, jiff_time, FunctionFlags::DETERMINISTIC)?;
    define_scalar_function(db, "jiff_time", 4, jiff_time, FunctionFlags::DETERMINISTIC)?;
    define_scalar_function(db, "jiff_time_hour", 1, jiff_time_hour, FunctionFlags::DETERMINISTIC)?;
    define_scalar_function(db, "jiff_time_round", -1, jiff_time_round, FunctionFlags::DETERMINISTIC)?;
    define_scalar_function(
        db,
        "jiff_time_strptime",
        2,
        jiff_time_strptime,
        FunctionFlags::UTF8,
    )?;

    Ok(())
}
