use chrono::NaiveDate;
use proconio::*;

fn main() {
    input! {year: i32}
    println!(
        "{}",
        365 + NaiveDate::from_ymd_opt(year, 2, 29).is_some() as usize
    );
}
