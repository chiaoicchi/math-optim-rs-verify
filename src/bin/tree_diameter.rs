use graphia::{csr::Csr, tree::diameter_path};
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
    let abc: Vec<(usize, usize, u64)> = (0..n - 1)
        .map(|_| (parse!(usize), parse!(usize), parse!(u64)))
        .collect();

    let csr = Csr::from_undirected_weighted(n, &abc);
    let (d, path) = diameter_path(&csr);
    writeln!(stdout, "{} {}", d, path.len()).ok();
    for &p in &path {
        write!(stdout, "{} ", p).ok();
    }
}
