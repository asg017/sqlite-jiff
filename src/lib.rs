use jiff::Unit;
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, define_scalar_function, Result};

// jiff_version() -> 'v0.1.0'
pub fn jiff_version(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(context, format!("v{}", env!("CARGO_PKG_VERSION")))?;
    Ok(())
}

pub fn jiff_debug(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(
        context,
        format!(
            "Version: v{}
Source: {}
",
            env!("CARGO_PKG_VERSION"),
            env!("GIT_HASH")
        ),
    )?;
    Ok(())
}

use jiff::fmt::temporal::DateTimeParser;

static PARSER: DateTimeParser = DateTimeParser::new();

pub fn jiff_duration(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let a = PARSER.parse_zoned(api::value_blob(&values[0])).unwrap();
    let b = PARSER.parse_zoned(api::value_blob(&values[1])).unwrap();

    let unit = match api::value_text(&values[2])? {
        "milisecond" | "milliseconds" | "ms" => Unit::Millisecond,
        "second" | "seconds" | "s" => Unit::Second,
        "minutes" | "minute" => Unit::Minute,
        "day" | "days" => Unit::Day,
        "hour" | "hours" | "hr" => Unit::Hour,
        _ => todo!(),
    };
    let span = &a - &b;
    api::result_double(context, span.total(unit).unwrap());
    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_jiff_init(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(
        db,
        "jiff_version",
        0,
        jiff_version,
        FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "jiff_debug",
        0,
        jiff_debug,
        FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC,
    )?;

    define_scalar_function(db, "jiff_duration", 3, jiff_duration, FunctionFlags::UTF8)?;
    Ok(())
}
