use math::mod_log_with_lower_bound_constraint;
use proconio::input;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {k: i64}

        // (2/9)(10^n - 1) = 0 (mod k)
        let res = if k % 2 == 1 {
            // 10^n = 1 (mod 9k)
            mod_log_with_lower_bound_constraint(10, 1, 9 * k, 0)
        } else {
            // 10^n = 1 (mod 9k/2)
            mod_log_with_lower_bound_constraint(10, 1, 9 * k / 2, 0)
        };

        if let Some(res) = res {
            println!("{}", res)
        } else {
            println!("-1")
        }
    }
}
