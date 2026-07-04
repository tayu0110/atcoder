use proconio::*;

fn main() {
    input! {n: usize}

    let mut dpp = vec![String::new(); n + 1];
    dpp[1] = "1".to_owned();
    if n >= 11 {
        dpp[11] = "11".to_owned();
    }
    if n >= 111 {
        dpp[111] = "111".to_owned();
    }
    if n >= 1111 {
        dpp[1111] = "1111".to_owned();
    }
    let mut dpm = dpp.clone();
    for i in 2..=n {
        for j in 1..i {
            let k = i - j;
            if dpp[i].is_empty() || dpp[j].len() + dpp[k].len() + 1 < dpp[i].len() {
                dpp[i] = format!("{}+{}", dpp[j], dpp[k]);
            }
            if dpm[j].len() + dpm[k].len() + 1 < dpp[i].len()
                && !dpm[j].is_empty()
                && !dpm[k].is_empty()
            {
                dpp[i] = format!("{}+{}", dpm[j], dpm[k]);
            }
            if dpp[j].len() + dpm[k].len() + 1 < dpp[i].len() && !dpm[k].is_empty() {
                dpp[i] = format!("{}+{}", dpp[j], dpm[k]);
            }
            if dpm[j].len() + dpp[k].len() + 1 < dpp[i].len() && !dpm[j].is_empty() {
                dpp[i] = format!("{}+{}", dpm[j], dpp[k]);
            }

            if j > 1 && i % j == 0 {
                let k = i / j;
                if dpm[i].is_empty() || dpp[j].len() + dpp[k].len() + 5 < dpm[i].len() {
                    dpm[i] = format!("({})*({})", dpp[j], dpp[k]);
                }
                if dpm[j].len() + dpm[k].len() + 1 < dpm[i].len()
                    && !dpm[j].is_empty()
                    && !dpm[k].is_empty()
                {
                    dpm[i] = format!("{}*{}", dpm[j], dpm[k]);
                }
                if dpp[j].len() + dpm[k].len() + 3 < dpm[i].len() && !dpm[k].is_empty() {
                    dpm[i] = format!("({})*{}", dpp[j], dpm[k]);
                }
                if dpm[j].len() + dpp[k].len() + 3 < dpm[i].len() && !dpm[j].is_empty() {
                    dpm[i] = format!("{}*({})", dpm[j], dpp[k]);
                }
            }
        }
    }

    if dpp[n].len() < dpm[n].len() || dpm[n].is_empty() {
        println!("{}", dpp[n])
    } else {
        println!("{}", dpm[n]);
    }
}
