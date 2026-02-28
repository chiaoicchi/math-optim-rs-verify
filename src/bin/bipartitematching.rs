use graphia::flow::{ResidualGraph, dinic};
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

    let l: usize = parse!(usize);
    let r: usize = parse!(usize);
    let m: usize = parse!(usize);

    let mut ab: Vec<(usize, usize, u32)> = Vec::with_capacity(m + l + r);
    for _ in 0..m {
        let a = parse!(usize);
        let b = parse!(usize);
        ab.push((a, l + b, 1));
    }
    for i in 0..l {
        ab.push((l + r, i, 1));
    }
    for i in 0..r {
        ab.push((l + i, l + r + 1, 1));
    }
    let mut residual_graph = ResidualGraph::from_directed(l + r + 2, &ab);
    let f = dinic(&mut residual_graph, l + r, l + r + 1, u32::MAX);
    writeln!(stdout, "{}", f).ok();
    for (i, (a, b, _)) in ab[..m].iter().enumerate() {
        if residual_graph.flow(i) == 1 {
            writeln!(stdout, "{} {}", a, b - l).ok();
        }
    }
}
