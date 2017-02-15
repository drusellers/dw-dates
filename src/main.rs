#![feature(plugin)]
#![plugin(clippy)]

//! Data Warehousing Tools
//!
//! A tool for generating date dimensions

extern crate chrono;
extern crate csv;
extern crate rustc_serialize;

use chrono::{Duration, UTC, TimeZone, Datelike, Date};
use rustc_serialize::Encodable;

impl Encodable for chrono::Weekday {}

#[derive(Debug, RustcEncodable)]
pub struct DateRow {
    // pub date: Date<UTC>,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub day_of_year: u32,
    pub weekday: chrono::Weekday,
    pub quarter: u16,
    pub month_name: &'static str,
    pub weeknum: u32,
}

// new weeks starts on sunday
fn weeknum(input: Date<UTC>) -> u32 {
    let ord = input.ordinal();
    let dow = input.weekday().num_days_from_sunday();
    let dow_jan_1 = UTC.ymd(input.year(), 1, 1).weekday().num_days_from_sunday();
    let weeknum = (ord + 6) / 7;
    if dow < dow_jan_1 {
        return weeknum + 1;
    }
    weeknum
}

fn quarter(input: Date<UTC>) -> u16 {
    match input.month() {
        1 | 2 | 3 => 1,
        4 | 5 | 6 => 2,
        7 | 8 | 9 => 3,
        _ => 4,
    }
}

fn month_name(input: Date<UTC>) -> &'static str {
    match input.month() {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        _ => "December",
    }
}

impl DateRow {
    fn convert(input: Date<UTC>) -> DateRow {
        DateRow {
            // date: input,
            year: input.year(),
            month: input.month(),
            day: input.day(),
            day_of_year: input.ordinal(),
            weekday: input.weekday(),
            quarter: quarter(input),
            month_name: month_name(input),
            weeknum: weeknum(input),
        }
    }
}
fn main() {

    let dt0 = UTC.ymd(2000, 1, 1);
    let dt1 = UTC.ymd(2000, 12, 31);

    let mut dt = dt0;
    let mut drs = Vec::new();
    while dt <= dt1 {
        let dr = DateRow::convert(dt);
        drs.push(dr);

        dt = dt.checked_add(Duration::days(1))
            .expect("Overflow happend!");
    }
    let mut w = csv::Writer::from_file("bob.csv").unwrap();
    w.encode(vec![("Year", "Month", "Day", "DayOfYear", "Quarter", "MonthName", "WeekNum")])
        .unwrap();
    for record in &drs {
        w.encode(record).unwrap();
    }
}
