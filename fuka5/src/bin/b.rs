use chrono::NaiveDateTime;
use proconio::*;

fn main() {
    loop {
        input! {ymd: String}

        if ymd == "0" {
            break;
        }

        input! {hms: String, time: String}

        let timestamp =
            NaiveDateTime::parse_from_str(format!("{ymd} {hms}").as_str(), "%Y/%m/%d %H:%M:%S")
                .unwrap()
                .timestamp()
                + (1 << time.len())
                - 1;
        let time = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
        println!("{}", time);
        // let fmt = format!("{}T{}", ymd.replace("/", "-"), hms.replace(":", ":"));
        // eprintln!("fmt: {fmt}");
        // let timestamp = DateTime::parse_from_rfc3339(fmt.as_str())
        //     .unwrap()
        //     .timestamp()
        //     + (1 << time.len())
        //     - 1;

        // let time = DateTime::from_timestamp(timestamp, 0).unwrap();
        // println!("{}", time.to_rfc3339().replace("-", "/").replace("T", " "));
    }
}
