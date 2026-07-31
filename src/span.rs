use jiff::{
    fmt::friendly::{Designator, Spacing, SpanPrinter}, RoundMode, Span, SpanRound, Unit
};
use sqlite_loadable::{api, define_scalar_function, prelude::*, Error, Result};

use crate::{
    date::{date_from_value, result_date},
    datetime::{datetime_from_value, result_datetime},
    time::{jiff_time_from_value, result_time, time_from_value},
    timestamp::timestamp_from_value,
    zoned::{jiff_zoned_from_value, result_zoned},
};

pub fn span_from_value(value: &*mut sqlite3_value) -> Result<Span> {
    let input = api::value_text(value)?;
    let x = input.parse();
    match x {
        Ok(x) => Ok(x),
        Err(e) => Err(Error::new_message(e.to_string())),
    }
}

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
        value => Err(Error::new_message(format!("Unknown unit {value}"))),
    }
}

fn jiff_span(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
  let span = span_from_value(&values[0])?;
  result_span(context, span)?;
  Ok(())
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
                    "none" => Spacing::None,
                    "between-units" => Spacing::BetweenUnits,
                    "between-units-and-designators" => Spacing::BetweenUnitsAndDesignators,
                    value => return Err(Error::new_message(format!("Unknown spacing '{value}'"))),
                };
                printer.spacing(spacing)
            }
            "designator" => {
                let designator = match api::value_text(&v)?.to_lowercase().as_str() {
                    "verbose" => Designator::Verbose,
                    "short" => Designator::Short,
                    "compact" => Designator::Compact,
                    value => return Err(Error::new_message(format!("Unknown designator {value}"))),
                };
                printer.designator(designator)
            }
            "direction" => {
                let direction = match api::value_text(&v)?.to_lowercase().as_str() {
                    "auto" => jiff::fmt::friendly::Direction::Auto,
                    "sign" => jiff::fmt::friendly::Direction::Sign,
                    "force-sign" => jiff::fmt::friendly::Direction::ForceSign,
                    "suffix" => jiff::fmt::friendly::Direction::Suffix,
                    value => return Err(Error::new_message(format!("Unknown direction {value}"))),
                };
                printer.direction(direction)
            }
            "comma-after-designator" => printer.comma_after_designator(api::value_int(&v) == 1),
            key => {
                return Err(Error::new_message(format!(
                    "Unknown key for jiff_span_format: '{key}'"
                )))
            }
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

fn result_span(context: *mut sqlite3_context, span: Span) -> Result<()> {
    let mut buf = String::new();
    SPAN_PRINTER.print_span(&span, &mut buf).unwrap();
    api::result_text(context, buf)?;
    Ok(())
}

fn jiff_span_round(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    if values.is_empty() {
        return Err(Error::new_message(
            "jiff_span_round requires at least one argument",
        ));
    }

    let span = span_from_value(&values[0])?;
    let mut options: SpanRound<'static> = SpanRound::new();
    if values.len() == 2 {
        let unit = unit_from_value(&values[1])?;
        result_span(context, span.round(unit).unwrap())?;
        return Ok(());
    }
    if values.len() % 2 != 1 {
        return Err(Error::new_message(
            "jiff_span_round() requires an even number of arguments after the span",
        ));
    }
    if values.len() == 3 && api::value_type(&values[1]) == api::ValueType::Integer {
        let x = api::value_int64(&values[1]);
        let unit = unit_from_value(&values[2])?;
        result_span(context, span.round((unit, x)).unwrap())?;
        return Ok(());
    }

    for pair in values[1..].iter().as_slice().chunks(2) {
        let k = pair[0];
        let v = pair[1];
        let key = api::value_text(&k)?;
        options = match key {
            "largest" => options.largest(unit_from_value(&v)?),
            "smallest" => options.smallest(unit_from_value(&v)?),
            "increment" => options.increment(api::value_int64(&v)),
            "mode" => options.mode(roundmode_from_value(&v)?),
            "relative" => {
              // TODO support &Zoned here somehow??
              if let Ok(datetime) = datetime_from_value(&v) {
                  options.relative(datetime)
              } else if let Ok(date) = date_from_value(&v) {
                  options.relative(date)
              } else {
                  return Err(Error::new_message("Invalid value for 'relative' option"));
              }
            }
            k => {
                return Err(Error::new_message(format!(
                    "Unknown key for jiff_span_round: '{k}'"
                )))
            }
        }
    }
    result_span(context, span.round(options).unwrap())?;
    Ok(())
}

fn roundmode_from_value(value: &*mut sqlite3_value) -> Result<RoundMode> {
    let input = api::value_text(value)?;
    match input.to_lowercase().as_str() {
        "ceil" => Ok(RoundMode::Ceil),
        "floor" => Ok(RoundMode::Floor),
        "expand" => Ok(RoundMode::Expand),
        "trunc" => Ok(RoundMode::Trunc),
        "half-ceil" => Ok(RoundMode::HalfCeil),
        "half-floor" => Ok(RoundMode::HalfFloor),
        "half-expand" => Ok(RoundMode::HalfExpand),
        "half-trunc" => Ok(RoundMode::HalfTrunc),
        "half-even" => Ok(RoundMode::HalfEven),
        value => Err(Error::new_message(format!("Unknown round mode {value}"))),
    }
}
fn jiff_span_total(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    // TODO:
    // - [ ] validate inputs
    // - [ ] (span, unit, date/datetime/zoned)
    // - [ ] (span, key1, value1, key2, ...)
    let span = span_from_value(&values[0])?;
    let unit = unit_from_value(&values[1])?;
    let result = if values.len() == 3 {
    span
          .total((unit, date_from_value(&values[2])?))
          .map_err(|e| Error::new_message(e.to_string()))?
    }else {
        span
          .total(unit)
          .map_err(|e| Error::new_message(e.to_string()))?
    };
      api::result_double(context, result);
    
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
        return Ok(());
    }
    if let Ok(date) = date_from_value(&values[0]) {
        result_date(context, date.saturating_add(span))?;
        return Ok(());
    }
    if let Ok(time) = jiff_time_from_value(&values[0]) {
        result_time(context, time.saturating_add(span))?;
        return Ok(());
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
        "jiff_span",
        1,
        jiff_span,
        FunctionFlags::DETERMINISTIC,
    )?;
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
    define_scalar_function(db, "jiff_add", 2, jiff_add, FunctionFlags::DETERMINISTIC)?;

    define_scalar_function(
        db,
        "jiff_span_round",
        -1,
        jiff_span_round,
        FunctionFlags::DETERMINISTIC,
    )?;
    // 4 possible args,
    for argc in [1, 3, 5, 7, 9] {
        define_scalar_function(
            db,
            "jiff_span_format",
            argc,
            jiff_span_format,
            FunctionFlags::DETERMINISTIC,
        )?;
    }

    define_scalar_function(
        db,
        "jiff_span_total",
        -1,
        jiff_span_total,
        FunctionFlags::DETERMINISTIC,
    )?;

    Ok(())
}
