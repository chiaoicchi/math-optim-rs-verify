use geome::point2d::{Point2D, convex_hull, p2};
use std::io::{BufWriter, Read, Write, stdin, stdout};

fn main() {
    let mut input = Vec::new();
    stdin().lock().read_to_end(&mut input).unwrap();
    let mut iter = input.split(|&b| b <= b' ').filter(|s| !s.is_empty());
    let mut stdout = BufWriter::new(stdout().lock());

    macro_rules! parse {
        (i64) => {{
            let s = iter.next().unwrap();
            let neg = s[0] == b'-';
            let mut x: i64 = 0;
            for &b in &s[neg as usize..] {
                x = x * 10 + (b - b'0') as i64;
            }
            if neg { -x } else { x }
        }};
        ($t:ty) => {{
            let s = iter.next().unwrap();
            let mut x: $t = 0;
            for &b in s {
                x = x * 10 + (b - b'0') as $t;
            }
            x
        }};
    }

    let t = parse!(u32);
    for _ in 0..t {
        let n = parse!(usize);
        let mut xy: Vec<Point2D<i64>> = (0..n).map(|_| p2(parse!(i64), parse!(i64))).collect();
        let ans = convex_hull(&mut xy);

        writeln!(stdout, "{}", ans.len()).ok();
        for point in ans.iter() {
            writeln!(stdout, "{} {}", point.x(), point.y()).ok();
        }
    }
}
