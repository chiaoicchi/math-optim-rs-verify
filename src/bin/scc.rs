use graphia::{csr::Csr, scc::kosaraju};
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

    let n: usize = parse!(usize);
    let m: usize = parse!(usize);

    let ab: Vec<(usize, usize)> = (0..m).map(|_| (parse!(usize), parse!(usize))).collect();

    let csr = Csr::from_directed_unweighted(n, &ab);
    let scc = kosaraju(&csr);
    let num_comp = scc.iter().max().map_or(0, |&x| x + 1);

    let mut offset = vec![0usize; num_comp + 1];
    for &c in &scc {
        offset[c + 1] += 1;
    }
    for i in 1..=num_comp {
        offset[i] += offset[i - 1];
    }
    let mut order = vec![0usize; n];
    let mut pos = offset.clone();
    for i in 0..n {
        let c = scc[i];
        order[pos[c]] = i;
        pos[c] += 1;
    }

    writeln!(stdout, "{}", num_comp).ok();
    for c in 0..num_comp {
        let start = offset[c];
        let end = offset[c + 1];
        write!(stdout, "{}", end - start).ok();
        for j in start..end {
            write!(stdout, " {}", order[j]).ok();
        }
        writeln!(stdout).ok();
    }
}
