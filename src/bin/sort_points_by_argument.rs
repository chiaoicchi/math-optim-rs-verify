use geome::vector2d::{Vector2D, v2};
use std::io::{BufWriter, Read, Write, stdin, stdout};

fn main() {
    let mut input = Vec::new();
    stdin().lock().read_to_end(&mut input).unwrap();
    let mut iter = input.split(|&b| b <= b' ').filter(|s| !s.is_empty());
    let mut stdout = BufWriter::new(stdout().lock());

    macro_rules! parse {
        (bytes) => {{ iter.next().unwrap() }};
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

    let n = parse!(usize);
    let mut xy: Vec<Vector2D<i64>> = (0..n).map(|_| v2(parse!(i64), parse!(i64))).collect();
    xy.sort_unstable_by(|a, b| {
        let az = *a == Vector2D::zero();
        let bz = *b == Vector2D::zero();
        match (az, bz) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => Vector2D::new(1i64, 0).arg_cmp_signed(b),
            (false, true) => a.arg_cmp_signed(&Vector2D::new(1, 0)),
            (false, false) => a.arg_cmp_signed(b),
        }
    });

    for v in xy.iter() {
        writeln!(stdout, "{} {}", v.x(), v.y()).ok();
    }
}
