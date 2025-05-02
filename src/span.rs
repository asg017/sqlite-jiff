use jiff::{
    fmt::friendly::{Designator, Spacing, SpanPrinter},
    Span, SpanRound, Unit,
};
use sqlite_loadable::{api, define_scalar_function, prelude::*, Error, Result};

use crate::{
    date::{date_from_value, result_date}, datetime::{datetime_from_value, result_datetime}, time::{jiff_time_from_value, result_time, time_from_value},
    timestamp::timestamp_from_value, zoned::{jiff_zoned_from_value, result_zoned},
};

pub fn unit_from_value(value: &*mut sqlite3_value) -> Result<Unit> {
    let input = api::value_text(value)?;
    match input.to_lowercase().as_str() {
        "nanosecond" | "nanoseconds" | "ns" => Ok(Unit::Nanosecond),
        "microsecond" | "microseconds" | "μs" | "us" => Ok(Unit::Microsecond),
        "millisecond" | "milliseconds" | "ms" => Ok(Unit::Millisecond),
        "second" | "seconds" | "s" => Ok(Unit::Second),
        "minute" | "minutes" => Ok(Unit::Minute),
        "hour" | "hours" | "hr" => Ok(Unit::Hour),
        "day" | "days" => Ok(Unit::Day),
        "week" | "weeks" => Ok(Unit::Week),
        "month" | "months" => Ok(Unit::Month),
        "year" | "years" => Ok(Unit::Year),
        _ => Err(Error::new_message("Unknown unit")),
    }
}

fn jiff_span_format(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let mut printer = SpanPrinter::new();
    let span = span_from_value(&values[0])?;
    for pair in values[1..].iter().as_slice().chunks(2) {
        let k = pair[0];
        let v = pair[1];
        let key = api::value_text(&k)?;
        printer = match key {
            "spacing" => {
                let spacing = match api::value_text(&v)?.to_lowercase().as_str() {
                    "between-units-and-designators" => Spacing::BetweenUnitsAndDesignators,
                    "between-units" => Spacing::BetweenUnits,
                    "none" => Spacing::None,
                    _ => return Err(Error::new_message("Unknown spacing")),
                };
                printer.spacing(spacing)
            }
            "designator" => {
                let designator = match api::value_text(&v)?.to_lowercase().as_str() {
                    "verbose" => Designator::Verbose,
                    "short" => Designator::Short,
                    "compact" => Designator::Compact,
                    _ => return Err(Error::new_message("Unknown designator")),
                };
                printer.designator(designator)
            }
            "direction" => {
                let direction = match api::value_text(&v)?.to_lowercase().as_str() {
                    "auto" => jiff::fmt::friendly::Direction::Auto,
                    "sign" => jiff::fmt::friendly::Direction::Sign,
                    "force-sign" => jiff::fmt::friendly::Direction::ForceSign,
                    "suffix" => jiff::fmt::friendly::Direction::Suffix,
                    _ => return Err(Error::new_message("Unknown direction")),
                };
                printer.direction(direction)
            }
            "comma-after-designator" => printer.comma_after_designator(api::value_int(&v) == 1),
            _ => todo!(),
        }
    }
    let mut buf = String::new();
    printer.print_span(&span, &mut buf).unwrap();
    api::result_text(context, buf)?;
    Ok(())
}

static SPAN_PRINTER: SpanPrinter = SpanPrinter::new()
    .spacing(Spacing::BetweenUnitsAndDesignators)
    .comma_after_designator(true)
    .designator(Designator::Verbose);

fn span_from_value(value: &*mut sqlite3_value) -> Result<Span> {
    let input = api::value_text(value)?;
    let x = input.parse();
    match x {
        Ok(x) => Ok(x),
        Err(e) => Err(Error::new_message(e.to_string())),
    }
}

fn result_span(context: *mut sqlite3_context, span: Span) -> Result<()> {
    let mut buf = String::new();
    SPAN_PRINTER.print_span(&span, &mut buf).unwrap();
    api::result_text(context, buf)?;
    Ok(())
}

fn jiff_span_round(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let span = span_from_value(&values[0])?;
    let mut round = SpanRound::new();
    for pair in values[1..].iter().as_slice().chunks(2) {
        let k = pair[0];
        let v = pair[1];
        let key = api::value_text(&k)?;
        round = match key {
            "largest" => round.largest(unit_from_value(&v)?),
            "smallest" => round.smallest(unit_from_value(&v)?),

            k => {
                return Err(Error::new_message(format!(
                    "Unknown key for jiff_span_round: '{k}'"
                )))
            }
        }
    }
    result_span(context, span.round(round).unwrap())?;
    Ok(())
}


fn jiff_span_total(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    // TODO:
    // - [ ] validate inputs
    // - [ ] (span, unit, date/datetime/zoned)
    // - [ ] (span, key1, value1, key2, ...)
    let span = span_from_value(&values[0])?;
    if values.len() == 2 {
      let unit = unit_from_value(&values[1])?;
      let result = span.total(unit).map_err(|e| Error::new_message(e.to_string()))?;
      api::result_double(context, result);
      return Ok(());
    }
    let mut round = SpanRound::new();
    for pair in values[1..].iter().as_slice().chunks(2) {
        let k = pair[0];
        let v = pair[1];
        let key = api::value_text(&k)?;
        round = match key {
            "largest" => round.largest(unit_from_value(&v)?),
            "smallest" => round.smallest(unit_from_value(&v)?),

            k => {
                return Err(Error::new_message(format!(
                    "Unknown key for jiff_span_round: '{k}'"
                )))
            }
        }
    }
    result_span(context, span.round(round).unwrap())?;
    Ok(())
}

fn jiff_add(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
  let span = span_from_value(&values[1])?;
  if let Ok(zoned) = jiff_zoned_from_value(&values[0]) {
    result_zoned(context, zoned.saturating_add(span))?;
    return Ok(());
  }
  if let Ok(datetime) = datetime_from_value(&values[0]) {
    result_datetime(context, datetime.saturating_add(span))?;
  }
  if let Ok(date) = date_from_value(&values[0]) {
    result_date(context, date.saturating_add(span))?;
  }
  if let Ok(time) = jiff_time_from_value(&values[0]) {
    result_time(context, time.saturating_add(span))?;
  }
  Ok(())
}

fn jiff_until(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    match (
        jiff_zoned_from_value(&values[0]),
        jiff_zoned_from_value(&values[1]),
    ) {
        (Ok(z1), Ok(z2)) => {
            match z1.until(&z2) {
                Ok(span) => {
                    result_span(context, span)?;
                }
                Err(_) => todo!(),
            }
            return Ok(());
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => (),
    };
    match (
        timestamp_from_value(&values[0]),
        timestamp_from_value(&values[1]),
    ) {
        (Ok(ts1), Ok(ts2)) => {
            let span = ts1.until(ts2).unwrap();
            result_span(context, span)?;
            return Ok(());
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => (),
    };
    match (
        datetime_from_value(&values[0]),
        datetime_from_value(&values[1]),
    ) {
        (Ok(dt1), Ok(dt2)) => {
            let span = dt1.until(dt2).unwrap();
            result_span(context, span)?;
            return Ok(());
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => (),
    };
    match (date_from_value(&values[0]), date_from_value(&values[1])) {
        (Ok(d1), Ok(d2)) => {
            let span = d1.until(d2).unwrap();
            result_span(context, span)?;
            return Ok(());
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => (),
    };
    match (time_from_value(&values[0]), time_from_value(&values[1])) {
        (Ok(t1), Ok(t2)) => {
            let span = t1.until(t2).unwrap();
            result_span(context, span)?;
            return Ok(());
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => (),
    };
    todo!();
}

fn jiff_since(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    match (
        jiff_zoned_from_value(&values[0]),
        jiff_zoned_from_value(&values[1]),
    ) {
        (Ok(z1), Ok(z2)) => {
            match z1.since(&z2) {
                Ok(span) => {
                    result_span(context, span)?;
                }
                Err(_) => todo!(),
            }
            return Ok(());
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => (),
    };
    match (
        timestamp_from_value(&values[0]),
        timestamp_from_value(&values[1]),
    ) {
        (Ok(ts1), Ok(ts2)) => {
            let span = ts1.since(ts2).unwrap();
            result_span(context, span)?;
            return Ok(());
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => (),
    };
    match (
        datetime_from_value(&values[0]),
        datetime_from_value(&values[1]),
    ) {
        (Ok(dt1), Ok(dt2)) => {
            let span = dt1.since(dt2).unwrap();
            result_span(context, span)?;
            return Ok(());
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => (),
    };
    match (date_from_value(&values[0]), date_from_value(&values[1])) {
        (Ok(d1), Ok(d2)) => {
            let span = d1.since(d2).unwrap();
            result_span(context, span)?;
            return Ok(());
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => (),
    };
    match (time_from_value(&values[0]), time_from_value(&values[1])) {
        (Ok(t1), Ok(t2)) => {
            let span = t1.since(t2).unwrap();
            result_span(context, span)?;
            return Ok(());
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => (),
    };
    todo!();
}

pub fn register(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(
        db,
        "jiff_until",
        2,
        jiff_until,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_since",
        2,
        jiff_since,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_add",
        2,
        jiff_add,
        FunctionFlags::DETERMINISTIC,
    )?;

    define_scalar_function(
        db,
        "jiff_span_round",
        -1,
        jiff_span_round,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_span_format",
        -1,
        jiff_span_format,
        FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_span_total",
        -1,
        jiff_span_total,
        FunctionFlags::DETERMINISTIC,
    )?;

    Ok(())
}
