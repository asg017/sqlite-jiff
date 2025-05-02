use jiff::{Timestamp, tz::{Offset, TimeZone, TimeZoneFollowingTransitions}};
use sqlite_loadable::{
    api,
    table::{ConstraintOperator, IndexInfo, VTab, VTabArguments, VTabCursor},
    BestIndexError, Result,
};
use sqlite_loadable::{prelude::*, Error};
use std::{mem, os::raw::c_int};

use crate::timestamp::result_timestamp;

static CREATE_SQL: &str = "CREATE TABLE x(timestamp, offset, dst, abbreviation, timezone hidden)";
enum Columns {
    Timestamp,
    Offset,
    Dst,
    Abbreviation,
    Timezone,
}

fn column(index: i32) -> Option<Columns> {
    match index {
        0 => Some(Columns::Timestamp),
        1 => Some(Columns::Offset),
        2 => Some(Columns::Dst),
        3 => Some(Columns::Abbreviation),
        4 => Some(Columns::Timezone),
        _ => None,
    }
}
#[repr(C)]
pub struct TimezoneTransitionsTable {
    base: sqlite3_vtab,
}


enum Idxnum {
  None = 0,
  HasAfter = 1,
  HasBefore = 2,
  HasBoth = 3,
}
impl<'vtab> VTab<'vtab> for TimezoneTransitionsTable {
    type Aux = ();
    type Cursor = TimezoneTransitionsCursor<'vtab>;

    fn connect(
        _db: *mut sqlite3,
        _aux: Option<&()>,
        _args: VTabArguments,
    ) -> Result<(String, TimezoneTransitionsTable)> {
        let base: sqlite3_vtab = unsafe { mem::zeroed() };
        let vtab = TimezoneTransitionsTable { base };
        // TODO db.config(VTabConfig::Innocuous)?;
        Ok((CREATE_SQL.to_owned(), vtab))
    }
    fn destroy(&self) -> Result<()> {
        Ok(())
    }

    fn best_index(&self, mut info: IndexInfo) -> core::result::Result<(), BestIndexError> {
        let mut has_before = false;
        let mut has_after = false;
        let mut has_tz = false;
        for mut constraint in info.constraints() {
          //constraint.set_argv_index(i);
            if constraint.usable() && matches!(column(constraint.column_idx()), Some(Columns::Timestamp)) {
              if constraint.op() == Some(ConstraintOperator::GT) {
                  has_after = true;
                  if has_before {

                  }
              } 
              if constraint.op() == Some(ConstraintOperator::LT) {
                  has_before = true;
              } 
            }
            if constraint.usable() && matches!(column(constraint.column_idx()), Some(Columns::Timezone)) && constraint.op() == Some(ConstraintOperator::EQ) {
                has_tz = true;
                constraint.set_omit(true);
                constraint.set_argv_index(0);
            }
        }
        if has_tz {
            return Err(BestIndexError::Error);
        }

        match (has_before, has_after) {
            (true, true) => info.set_idxnum(Idxnum::HasBoth as i32),
            (true, false) => info.set_idxnum(Idxnum::HasBefore as i32),
            (false, true) => info.set_idxnum(Idxnum::HasAfter as i32),
            (false, false) => info.set_idxnum(Idxnum::None as i32),
        }
        info.set_estimated_cost(100000.0);
        info.set_estimated_rows(100000);

        Ok(())
    }

    fn open(&mut self) -> Result<TimezoneTransitionsCursor> {
        Ok(TimezoneTransitionsCursor::new())
    }
}

#[repr(C)]
pub struct TimezoneTransitionsCursor<'a> {
    base: sqlite3_vtab_cursor,
    rowid: i64,
    tz: TimeZone,
    iter: Option<TimeZoneFollowingTransitions<'a>>,
    current: Option<Item>,
}
impl<'a> TimezoneTransitionsCursor<'_> {
    fn new<'vtab>() -> TimezoneTransitionsCursor<'vtab> {
        let base: sqlite3_vtab_cursor = unsafe { mem::zeroed() };
        TimezoneTransitionsCursor {
            base,
            rowid: 0,
            tz: TimeZone::UTC,
            iter: None,
            current: None,
        }
    }
}

unsafe fn extend_lifetime<'b>(
    r: TimeZoneFollowingTransitions<'b>,
) -> TimeZoneFollowingTransitions<'static> {
    std::mem::transmute::<TimeZoneFollowingTransitions<'b>, TimeZoneFollowingTransitions<'static>>(
        r,
    )
}

struct Item {
    ts: Timestamp,
    offset: Offset,
    abbreviation: String,
    dst: bool,
}

impl VTabCursor for TimezoneTransitionsCursor<'_> {
    fn filter(
        &mut self,
        idx_num: c_int,
        _idx_str: Option<&str>,
        values: &[*mut sqlite3_value],
    ) -> Result<()> {
        let tz_name = "America/Los_Angeles"; //api::value_text(values.get(0).unwrap())?;
        let tz = TimeZone::get(tz_name).unwrap();

        if idx_num == Idxnum::None as i32 {
            // no constraints, just return all transitions
            self.tz = tz;
            unsafe {
                self.iter = Some(extend_lifetime(self.tz.following(Timestamp::now())));
            }
            self.rowid = 0;
            return self.next();
        }
        self.tz = tz;
        //let x: jiff::tz::TimeZoneFollowingTransitions<'_> =
        unsafe {
            self.iter = Some(extend_lifetime(self.tz.following(Timestamp::now())));
        }
        /*while let Some(transition) = x.next() {
          println!("{:?}", transition);
          transition.timestamp();
          transition.offset();
          transition.dst();
          transition.abbreviation();
        } */
        //self.iter = Some(start.series(span));
        self.rowid = 0;
        self.next()
    }

    fn next(&mut self) -> Result<()> {
        //self.current = self.iter.as_mut().unwrap().next();
        self.current = match self.iter.as_mut().unwrap().next() {
            Some(transition) => Some(Item {
                ts: transition.timestamp(),
                offset: transition.offset(),
                abbreviation: transition.abbreviation().to_string(),
                dst: transition.dst().is_dst(),
            }),
            None => None,
        };
        self.rowid += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.current.is_none()
    }

    fn column(&self, context: *mut sqlite3_context, i: c_int) -> Result<()> {
      let item = self.current.as_ref().ok_or_else(|| {
            Error::new_message("No current item in TimezoneTransitionsCursor")
        })?;
        match column(i) {
            Some(Columns::Timestamp) => result_timestamp(context, item.ts)?,
            Some(Columns::Offset) => api::result_text(context, item.offset.to_string())?,
            Some(Columns::Dst) => api::result_bool(context, item.dst),
            Some(Columns::Abbreviation) => api::result_text(context, item.abbreviation.clone())?,
            Some(Columns::Timezone) => (),
            None => todo!(),
        }
        Ok(())
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.rowid)
    }
}
