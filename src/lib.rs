mod date;
mod datetime;
mod jiff_datetime_series;
mod timezone_transitions;
mod span;
mod time;
mod timestamp;
mod zoned;

use sqlite_loadable::{api, define_scalar_function, prelude::*, Result};

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

    crate::time::register(db)?;
    crate::date::register(db)?;
    crate::datetime::register(db)?;
    crate::timestamp::register(db)?;
    crate::zoned::register(db)?;
    crate::span::register(db)?;
    Ok(())
}
