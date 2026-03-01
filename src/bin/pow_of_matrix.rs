use algebrae::{linear::Matrix, modular::Gf32};
use std::io::{BufWriter, Read, Write, stdin, stdout};

const MOD: u32 = 998_244_353;

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
    let k = parse!(u64);
    let a: Vec<Gf32<MOD>> = (0..n * n).map(|_| Gf32::new(parse!(u32))).collect();
    let mat_a = Matrix::from_flat(n, n, a);
    let ans = mat_a.pow(k);
    for row in ans.iter() {
        for i in 0..n {
            write!(stdout, "{} ", row[i]).ok();
        }
        writeln!(stdout).ok();
    }
}
