use std::fmt::Display;

fn main() {
    let a = "A\nB\nC\nA\nB\nB\nA";
    let b = "C\nB\nA\nB\nA\nC";
    let res = Diff.diff(a.to_string(), b.to_string(), Myers);
    println!("{}", DiffResult(res));
}

#[derive(Clone)]
struct Line {
    text: String,
    number: usize,
}

struct Diff;

enum Edit {
    Insert(Line),
    Delete(Line),
    Unchanged(Line, Line),
}

struct DiffResult(Vec<Edit>);

impl Display for DiffResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for edit in &self.0 {
            match edit {
                Edit::Insert(line) => writeln!(f, "+     {}  {}", line.number, line.text)?,
                Edit::Delete(line) => writeln!(f, "-  {}     {}", line.number, line.text)?,
                Edit::Unchanged(prev_line, new_line) => writeln!(
                    f,
                    "   {}  {}  {}",
                    prev_line.number, new_line.number, prev_line.text
                )?,
            }
        }

        Ok(())
    }
}

impl Diff {
    fn lines(&self, text: String) -> Vec<Line> {
        text.lines()
            .enumerate()
            .map(|l| Line {
                text: l.1.to_string(),
                number: l.0 + 1,
            })
            .collect::<Vec<_>>()
    }

    fn diff(&self, a: String, b: String, differ: impl Differ) -> Vec<Edit> {
        differ.diff(self.lines(a), self.lines(b))
    }
}

trait Differ {
    fn diff(&self, a: Vec<Line>, b: Vec<Line>) -> Vec<Edit>;
}

struct Myers;

impl Differ for Myers {
    fn diff(&self, a: Vec<Line>, b: Vec<Line>) -> Vec<Edit> {
        self.backtrack(&a, &b)
            .into_iter()
            .rev()
            .map(|(previous_x, previous_y, x, y)| {
                if x == previous_x {
                    Edit::Insert((&b[previous_y]).clone())
                } else if y == previous_y {
                    Edit::Delete((&a[previous_x]).clone())
                } else {
                    Edit::Unchanged((&a[previous_x]).clone(), (&b[previous_y]).clone())
                }
            })
            .collect::<Vec<_>>()
    }
}

impl Myers {
    fn shortest_edit(&self, a: &Vec<Line>, b: &Vec<Line>) -> Vec<Vec<usize>> {
        let n = a.len();
        let m = b.len();
        let max = n + m;
        let mut x: usize;
        let mut y: usize;
        let mut v: Vec<usize> = vec![0; max * 2 + 1];

        v[1 + max] = 0;

        let mut t: Vec<Vec<usize>> = vec![];

        for d in 0..=max as i32 {
            t.push(v.clone());

            for k in (-d..=d).step_by(2) {
                let ki = (k + max as i32) as usize;
                if k == -d || k != d && &v[ki - 1] < &v[ki + 1] {
                    x = v[ki + 1];
                } else {
                    x = v[ki - 1] + 1;
                }

                y = (x as i32 - k) as usize;

                while x < n && y < m && a[x].text == b[y].text {
                    x = x + 1;
                    y = y + 1;
                }

                v[ki] = x;

                // Should this be == instead?
                if x >= n && y >= m {
                    return t;
                }
            }
        }

        unreachable!()
    }

    fn backtrack(&self, a: &Vec<Line>, b: &Vec<Line>) -> Vec<(usize, usize, usize, usize)> {
        let mut x = a.len();
        let mut y = b.len();
        let max = x + y;
        let mut result = vec![];
        let mut previous_k;
        let mut previous_x;
        let mut previous_y;
        for (d, v) in self.shortest_edit(a, b).into_iter().enumerate().rev() {
            let d = d as i32;
            let k = x as i32 - y as i32;
            let ki = (k + max as i32) as usize;

            if k == -d || k != d && &v[ki - 1] < &v[ki + 1] {
                previous_k = k + 1;
            } else {
                previous_k = k - 1;
            }
            let previous_ki = (previous_k + max as i32) as usize;

            previous_x = v[previous_ki];
            previous_y = (previous_x as i32 - previous_k) as usize;

            while x > previous_x && y > previous_y {
                result.push((x - 1, y - 1, x, y));
                x -= 1;
                y -= 1;
            }

            if d > 0 {
                result.push((previous_x, previous_y, x, y));
            }

            x = previous_x;
            y = previous_y;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    // use crate::{Diff, Myers};

    // #[test]
    // fn example() {
    //     let a = "A\nB\nC\nA\nB\nB\nA";
    //     let b = "C\nB\nA\nB\nA\nC";

    //     let result = Myers.shortest_edit(&Diff.lines(a.to_string()), &Diff.lines(b.to_string()));

    //     assert_eq!(result, 5)
    // }
}
