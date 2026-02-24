use ac_library::MfGraph;
use proconio::{input, marker};

#[proconio::fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            s: [marker::Bytes; h],
        }
        let mut t = vec![vec![0; w + 2]; h + 2];
        for i in 0..h {
            t[i + 1][1..1 + w].clone_from_slice(&s[i]);
            t[i + 1][0] = b'x';
            t[i + 1][1 + w] = b'x';
        }
        t[0].fill(b'x');
        t[h + 1].fill(b'x');
        t[0][0] = b'.';
        t[0][1] = b'.';
        t[1][0] = b'.';
        t[h + 1][w + 1] = b'.';
        t[h][w + 1] = b'.';
        t[h + 1][w] = b'.';
        let h = h + 2;
        let w = w + 2;
        let s = t;
        let mut graph = MfGraph::new(2 * h * w);
        const INF: u64 = 30000;
        let off = h * w;
        let drc = [(0, 2), (0, 2usize.wrapping_neg()), (2, 0), (2usize.wrapping_neg(), 0)];
        for r in 0..h {
            for c in 0..w {
                if s[r][c] == b'.' { continue }
                graph.add_edge(r * w + c, off + r * w + c, if s[r][c] == b'x' { INF } else { 1 });
                for nr in r.saturating_sub(1)..=(r + 1).min(h - 1) {
                    for nc in c.saturating_sub(1)..=(c + 1).min(w - 1) {
                        if (nr, nc) == (r, c) || s[nr][nc] == b'.' { continue }
                        graph.add_edge(off + r * w + c, nr * w + nc, INF);
                    }
                }
                for (dr, dc) in drc {
                    let nr = r.wrapping_add(dr);
                    let nc = c.wrapping_add(dc);
                    if (0..h).contains(&nr) && (0..w).contains(&nc) && s[nr][nc] != b'.' {
                        graph.add_edge(off + r * w + c, nr * w + nc, INF);
                    }
                }
            }
        }
        let ans = graph.flow(w - 1, off + (h - 1) * w);
        assert!(ans <= 10000);
        println!("{ans}");
    }
}