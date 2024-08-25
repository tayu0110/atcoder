use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    loop {
        let x: i64 = rng.gen_range(-1_000_000_000, -500_000_000);
        let y: i64 = rng.gen_range(-1_000_000_000, 1_000_000_000);
        let h: i64 = rng.gen_range(500_000_000, 1_000_000_000);
        let w: i64 = rng.gen_range(-1_000_000_000, 1_000_000_000);
        let rxy: i64 = rng.gen_range(900_000_000, 1_000_000_000);
        let rhw: i64 = rng.gen_range(900_000_000, 1_000_000_000);

        eprintln!(
            "x: {}, y: {}, h: {}, w: {}, rxy: {}, rhw: {}",
            x, y, h, w, rxy, rhw
        );
        let d = (x - h) * (x - h) + (y - w) * (y - w);
        let d2 = (x as f64 - h as f64) * (x as f64 - h as f64)
            + (y as f64 - w as f64) * (y as f64 - w as f64);
        let r_low = (rxy - rhw) * (rxy - rhw);
        let r_hi = (rxy + rhw) * (rxy + rhw);
        assert_eq!(
            r_low <= d && d <= r_hi,
            r_low as f64 <= d2 && d2 <= r_hi as f64
        );
    }
}
