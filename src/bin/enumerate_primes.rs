use algebrae::num_theory::SieveEratosthenes;
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

    let n = parse!(usize);
    let a = parse!(usize);
    let b = parse!(usize);

    let eratos = SieveEratosthenes::new(n);
    let primes = eratos.primes();

    writeln!(
        stdout,
        "{} {}",
        primes.len(),
        if primes.len() < b {
            0
        } else {
            (primes.len() - b + a - 1) / a
        }
    )
    .ok();
    for &prime in primes.iter().skip(b).step_by(a) {
        write!(stdout, "{} ", prime).ok();
    }
    writeln!(stdout).ok();
}
