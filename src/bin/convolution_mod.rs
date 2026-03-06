use algebrae::{conv::ntt::multiply32, modular::Gf32};
use std::io::{BufWriter, Read, Write, stdin, stdout};

const P: u32 = 998_244_353;

fn main() {
    let mut input = Vec::new();
    stdin().lock().read_to_end(&mut input).unwrap();
    let mut iter = input.split(|&b| b <= b' ').filter(|s| !s.is_empty());
    let mut stdout = BufWriter::new(stdout().lock());

    macro_rules! parse {
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
    let m = parse!(usize);
    let a: Vec<Gf32<P>> = (0..n).map(|_| Gf32::new(parse!(u32))).collect();
    let b: Vec<Gf32<P>> = (0..m).map(|_| Gf32::new(parse!(u32))).collect();

    let c = multiply32(a, b);
    for c in c.iter() {
        write!(stdout, "{} ", c).ok();
    }
}
