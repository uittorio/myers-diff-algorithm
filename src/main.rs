fn main() {}

struct Line {
    text: String,
    number: usize,
}

struct Diff;

impl Diff {
    fn lines(&self, text: String) -> Vec<Line> {
        text.lines()
            .enumerate()
            .map(|l| Line {
                text: l.1.to_string(),
                number: l.0,
            })
            .collect::<Vec<_>>()
    }

    fn diff(&self, a: String, b: String, differ: impl Differ) {
        differ.diff(self.lines(a), self.lines(b))
    }
}

trait Differ {
    fn diff(&self, a: Vec<Line>, b: Vec<Line>);
}

struct Myers;

impl Differ for Myers {
    fn diff(&self, a: Vec<Line>, b: Vec<Line>) {
        todo!()
    }
}

impl Myers {
    fn shortest_edit(&self, a: &Vec<Line>, b: &Vec<Line>) -> usize {
        let n = a.len();
        let m = b.len();
        let max = n + m;
        let mut x: usize;
        let mut y: usize;
        let mut v: Vec<usize> = vec![0; max * 2 + 1];

        v[1] = 0;

        for d in 0..max as i32 {
            for k2 in (-d..d).step_by(2) {
                let k = (k2 + max as i32) as usize;
                if k2 == -d || k2 != d && &v[k - 1] < &v[k + 1] {
                    x = v[k + 1];
                } else {
                    x = v[k - 1] + 1;
                }

                y = (x as i32 - k2) as usize;

                while x < n && y < m && a[x].text == b[y].text {
                    x = x + 1;
                    y = y + 1;
                }

                v[k] = x;

                if x == n && y == m {
                    return d as usize;
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Diff, Myers};

    #[test]
    fn example() {
        let a = "A\nB\nC\nA\nB\nB\nA";
        let b = "C\nB\nA\nB\nA\nC";

        let result = Myers.shortest_edit(&Diff.lines(a.to_string()), &Diff.lines(b.to_string()));

        assert_eq!(result, 5)
    }
}
