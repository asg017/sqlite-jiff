use jiff::civil::{DateTime, DateTimeSeries};
use jiff::Span;
use sqlite_loadable::{
    api,
    table::{ConstraintOperator, IndexInfo, VTab, VTabArguments, VTabCursor},
    BestIndexError, Result,
};
use sqlite_loadable::{prelude::*, Error};
use std::{mem, os::raw::c_int};

use crate::datetime::result_datetime;

static CREATE_SQL: &str = "CREATE TABLE x(datetime, start hidden, period hidden)";
enum Columns {
    Datetime,
    Start,
    Period,
}

fn column(index: i32) -> Option<Columns> {
    match index {
        0 => Some(Columns::Datetime),
        1 => Some(Columns::Start),
        2 => Some(Columns::Period),
        _ => None,
    }
}
#[repr(C)]
pub struct DatetimeSeriesTable {
    base: sqlite3_vtab,
}

impl<'vtab> VTab<'vtab> for DatetimeSeriesTable {
    type Aux = ();
    type Cursor = DatetimeSeriesCursor;

    fn connect(
        _db: *mut sqlite3,
        _aux: Option<&()>,
        _args: VTabArguments,
    ) -> Result<(String, DatetimeSeriesTable)> {
        let base: sqlite3_vtab = unsafe { mem::zeroed() };
        let vtab = DatetimeSeriesTable { base };
        // TODO db.config(VTabConfig::Innocuous)?;
        Ok((CREATE_SQL.to_owned(), vtab))
    }
    fn destroy(&self) -> Result<()> {
        Ok(())
    }

    fn best_index(&self, mut info: IndexInfo) -> core::result::Result<(), BestIndexError> {
        let mut has_start = false;
        let mut has_period = false;
        for mut constraint in info.constraints() {
            match column(constraint.column_idx()) {
                Some(Columns::Start) => {
                    if constraint.usable() && constraint.op() == Some(ConstraintOperator::EQ) {
                        constraint.set_omit(true);
                        constraint.set_argv_index(1);
                        has_start = true;
                    } else {
                        return Err(BestIndexError::Constraint);
                    }
                }
                Some(Columns::Period) => {
                    if constraint.usable() && constraint.op() == Some(ConstraintOperator::EQ) {
                        constraint.set_omit(true);
                        constraint.set_argv_index(2);
                        has_period = true;
                    } else {
                        return Err(BestIndexError::Constraint);
                    }
                }

                _ => (),
            }
        }
        if !has_start || !has_period {
            return Err(BestIndexError::Error);
        }
        info.set_estimated_cost(100000.0);
        info.set_estimated_rows(100000);
        info.set_idxnum(2);

        Ok(())
    }

    fn open(&mut self) -> Result<DatetimeSeriesCursor> {
        Ok(DatetimeSeriesCursor::new())
    }
}

#[repr(C)]
pub struct DatetimeSeriesCursor {
    base: sqlite3_vtab_cursor,
    rowid: i64,
    iter: Option<DateTimeSeries>,
    current: Option<DateTime>,
}
impl DatetimeSeriesCursor {
    fn new<'vtab>() -> DatetimeSeriesCursor {
        let base: sqlite3_vtab_cursor = unsafe { mem::zeroed() };
        DatetimeSeriesCursor {
            base,
            rowid: 0,
            iter: None,
            current: None,
        }
    }
}

impl VTabCursor for DatetimeSeriesCursor {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        values: &[*mut sqlite3_value],
    ) -> Result<()> {
        let start: DateTime = api::value_text(&values[0])?.parse().unwrap();
        let span: Span = api::value_text(&values[1])?.parse().unwrap();
        if span.is_zero() {
            return Err(Error::new_message(
                "jiff_datetime_series span cannot be zero",
            ));
        }
        DateTime::MAX.series(Span::new());
        self.iter = Some(start.series(span));
        self.rowid = 0;
        self.next()
    }

    fn next(&mut self) -> Result<()> {
      if let Some(iter)  = self.iter.as_mut(){ 
        self.current = iter.next();
        self.rowid += 1;
      }else {
        self.current = None;
      }
      Ok(())
    }

    fn eof(&self) -> bool {
        self.current.is_none()
    }

    fn column(&self, context: *mut sqlite3_context, i: c_int) -> Result<()> {
        match column(i) {
            Some(Columns::Start) => api::result_null(context),
            Some(Columns::Period) => api::result_null(context),
            Some(Columns::Datetime) =>  {
                if let Some(dt) = &self.current {
                  result_datetime(context, *dt)?;
                }
            },
            None => (),
        }
        Ok(())
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.rowid)
    }
}
