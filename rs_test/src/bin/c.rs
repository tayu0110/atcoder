fn main() {
    let n: usize = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        s.parse().unwrap()
    };
    let mut p = vec![];
    for _ in 0..n {
        let (x, y): (i64, i64) = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let mut ws = s.split_whitespace();
            (ws.next().unwrap().parse().unwrap(), ws.next().unwrap().parse().unwrap())
        };
        p.push((x, y));
    }
    let t = p.clone();
    p.sort();
    let mut res = 0;
    let mut map = std::collections::HashMap::new();
    while !p.is_empty() {
        res += 1;
        let buf = geometry::convex_hull(&p);
        // eprintln!("buf: {:?}", buf);
        buf.into_iter().for_each(|v| { map.insert(v, res); });
        p = p.into_iter().filter(|v| !map.contains_key(v)).collect::<Vec<(i64, i64)>>();
        // eprintln!("p: {:?}", p);
    }
    for v in t {
        println!("{}", map.get(&v).unwrap());
    }
}

#[allow(dead_code)]
mod geometry {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    struct Vector(i64, i64);
    impl From<[i64; 2]> for Vector { fn from(from: [i64; 2]) -> Self { Self(from[0], from[1]) } }
    impl std::convert::TryFrom<Vec<i64>> for Vector {
        type Error = Error;
        fn try_from(value: Vec<i64>) -> Result<Self, Self::Error> {
            match value.len() {
                2 => Ok(Self(value[0], value[1])),
                _ => Err(Error("Failed to generate the instance of geometry::Vector because the length of the argument Vec<i64> is not 2."))
            }
        }
    }
    impl Vector {
        fn new(from: [i64; 2], to: [i64; 2]) -> Self { Self(to[0] - from[0], to[1] - from[1]) }
        fn inner_product(&self, rhs: &Vector) -> i64 { self.0 * rhs.0 + self.1 * rhs.1 }
        fn exterior_product(&self, rhs: &Vector) -> i64 { self.0 * rhs.1 - self.1 * rhs.0 }
        fn scalar_product(&self, rhs: i64) -> Self { Self(self.0 * rhs, self.1 * rhs) }
    }
    impl std::ops::Add for Vector { type Output = Vector; fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0, self.1 + rhs.1) } }
    impl std::ops::Sub for Vector { type Output = Vector; fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0, self.1 - rhs.1) } }
    impl std::ops::Neg for Vector { type Output = Vector; fn neg(self) -> Self::Output { Self(-self.0, -self.1) } }
    impl std::ops::AddAssign for Vector { fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; self.1 += rhs.1; } }
    impl std::ops::SubAssign for Vector { fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; self.1 -= rhs.1; } }
    struct Error(&'static str);

    pub fn convex_hull(points: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        if points.len() < 2 {
            return points.clone();
        }
        let mut points = points.clone();
        points.sort();
        let (mut upper, mut lower) = (vec![], vec![]);
        for (x, y) in points {
            while upper.len() >= 2 {
                let ((sx, sy), (fx, fy)) = (upper.pop().unwrap(), upper.pop().unwrap());
                let (f, s) = (Vector::new([fx, fy], [sx, sy]), Vector::new([sx, sy], [x, y]));
                let exterior_product = f.exterior_product(&s);
                upper.push((fx, fy));
                if exterior_product <= 0 {
                    upper.push((sx, sy));
                    break;
                }
            }
            upper.push((x, y));
            while lower.len() >= 2 {
                let ((sx, sy), (fx, fy)) = (lower.pop().unwrap(), lower.pop().unwrap());
                let (f, s) = (Vector::new([fx, fy], [sx, sy]), Vector::new([sx, sy], [x, y]));
                let exterior_product = f.exterior_product(&s);
                lower.push((fx, fy));
                if exterior_product >= 0 {
                    lower.push((sx, sy));
                    break;
                }
            }
            lower.push((x, y));
        }
        lower.pop().unwrap();
        lower.reverse();
        upper.append(&mut lower);
        upper.pop().unwrap();
        upper
    }
}