fn main() {
}

struct Line {
    text: String,
    number: usize
}

struct Diff;

impl Diff {
    fn lines(&self, text: String) -> Vec<Line> {
        text.lines().enumerate().map(|l|  Line{text: l.1.to_string(), number: l.0}).collect::<Vec<_>>()
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
