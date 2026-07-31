use jiff::civil::{Date, DateSeries};
use jiff::Span;
use sqlite_loadable::{
    api,
    table::{ConstraintOperator, IndexInfo, VTab, VTabArguments, VTabCursor},
    BestIndexError, Result,
};
use sqlite_loadable::{prelude::*, Error};
use std::{mem, os::raw::c_int};

use crate::date::{date_from_value, result_date};
use crate::span::span_from_value;

static CREATE_SQL: &str = "CREATE TABLE x(date, start hidden, period hidden)";
enum Columns {
    Date,
    Start,
    Period,
}

fn column(index: i32) -> Option<Columns> {
    match index {
        0 => Some(Columns::Date),
        1 => Some(Columns::Start),
        2 => Some(Columns::Period),
        _ => None,
    }
}
#[repr(C)]
pub struct DateSeriesTable {
    base: sqlite3_vtab,
}

impl<'vtab> VTab<'vtab> for DateSeriesTable {
    type Aux = ();
    type Cursor = DateSeriesCursor;

    fn connect(
        _db: *mut sqlite3,
        _aux: Option<&()>,
        _args: VTabArguments,
    ) -> Result<(String, DateSeriesTable)> {
        let base: sqlite3_vtab = unsafe { mem::zeroed() };
        let vtab = DateSeriesTable { base };
        // TODO db.config(VTabConfig::Innocuous)?;
        Ok((CREATE_SQL.to_owned(), vtab))
    }
    fn destroy(&self) -> Result<()> {
        Ok(())
    }

    fn best_index(&self, mut info: IndexInfo) -> core::result::Result<(), BestIndexError> {
        let mut has_start = false;
        let mut has_period = false;
        let mut idxstr = String::new();
        let mut argv_extra = 3;
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
                Some(Columns::Date) => {
                    if !constraint.usable() {
                      continue;
                        
                    }
                    // TODO set_omit(true) once filter() actually applies
                    // these constraints; until then SQLite must re-check them
                    match constraint.op() {
                      Some(ConstraintOperator::GT)  => {
                        idxstr.push_str("A");
                        constraint.set_argv_index(argv_extra);
                        argv_extra +=1;
                      }
                      Some(ConstraintOperator::GE)  => {
                        idxstr.push_str("B");
                        constraint.set_argv_index(argv_extra);
                        argv_extra +=1;
                      }
                      Some(ConstraintOperator::LT)  => {
                        idxstr.push_str("C");
                        constraint.set_argv_index(argv_extra);
                        argv_extra +=1;
                      }
                      Some(ConstraintOperator::LE)  => {
                        idxstr.push_str("D");
                        constraint.set_argv_index(argv_extra);
                        argv_extra +=1;
                      }
                      _ => (),
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
        info.set_idxstr(idxstr.as_str()).map_err(|_| BestIndexError::Error)?;

        Ok(())
    }

    fn open(&mut self) -> Result<DateSeriesCursor> {
        Ok(DateSeriesCursor::new())
    }
}

#[allow(dead_code)]
enum ConstraintType {
  GT,
  GE,
  LT,
  LE,
}
#[allow(dead_code)]
struct Constraint {
    value: Date,
    constraint_type: ConstraintType,
}

#[repr(C)]
pub struct DateSeriesCursor {
    base: sqlite3_vtab_cursor,
    rowid: i64,
    iter: Option<DateSeries>,
    current: Option<Date>,
}
impl DateSeriesCursor {
    fn new<'vtab>() -> DateSeriesCursor {
        let base: sqlite3_vtab_cursor = unsafe { mem::zeroed() };
        DateSeriesCursor {
            base,
            rowid: 0,
            iter: None,
            current: None,
        }
    }
}


impl VTabCursor for DateSeriesCursor {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        values: &[*mut sqlite3_value],
    ) -> Result<()> {
        let start = date_from_value(&values[0])?;
        let span: Span = span_from_value(&values[1])?;
        // TODO apply the GT/GE/LT/LE constraints passed in values[2..]
        // (keyed by idx_str), so best_index can set_omit them
        /*for (idx, extra) in values.iter().skip(2).enumerate() {
            let x = date_from_value(extra)?;
            match idx_str.unwrap().get(idx) {
              Some("A")  => {
                let value = date
              }
            }
        }*/
        if span.is_zero() {
            return Err(Error::new_message(
                "jiff_date_series span cannot be zero",
            ));
        }
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
            Some(Columns::Date) =>  {
                if let Some(date) = &self.current {
                  result_date(context, *date)?;
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
