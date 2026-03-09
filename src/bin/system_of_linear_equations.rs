use algebrae::{
    linear::{Matrix, linear_system},
    num_theory::Gf,
};
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
    let m = parse!(usize);
    let a: Vec<Gf<MOD>> = (0..n * m).map(|_| Gf::new(parse!(u32))).collect();
    let b: Vec<Gf<MOD>> = (0..n).map(|_| Gf::new(parse!(u32))).collect();
    let mat_a = Matrix::from_flat(n, m, a);
    if let Some((sol, kernel)) = linear_system(&mat_a, &b) {
        let r = kernel.h();
        writeln!(stdout, "{}", r).ok();
        for c in sol {
            write!(stdout, "{} ", c).ok();
        }
        writeln!(stdout).ok();
        for v in kernel.iter() {
            for d in v {
                write!(stdout, "{} ", d).ok();
            }
            writeln!(stdout).ok();
        }
    } else {
        writeln!(stdout, "-1").ok();
    }
}
