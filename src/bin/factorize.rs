use algebrae::num_theory::factorize;
use std::io::{BufWriter, Read, Write, stdin, stdout};

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

    let q = parse!(u32);

    for _ in 0..q {
        let f = factorize(parse!(u64));
        let mut len = 0;
        for f in &f {
            len += f.1;
        }
        write!(stdout, "{} ", len).ok();
        for (f, c) in &f {
            for _ in 0..*c {
                write!(stdout, "{} ", f).ok();
            }
        }
        writeln!(stdout).ok();
    }
}
