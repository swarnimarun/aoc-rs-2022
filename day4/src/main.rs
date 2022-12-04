fn main() {
    println!("--------AOC-2022-DAY-4-------");
    let src = std::fs::read_to_string("./day4/input").unwrap_or_default();
    println!("P1-> {}", part_1(&src));
    println!("P2-> {}", part_2(&src));
}

#[derive(Debug, Clone, Copy)]
struct Range(usize, usize);
impl Range {
    fn complete_overlap(&self, v: &Self) -> bool {
        self.0 <= v.0 && v.1 <= self.1
    }
    fn either_complete_overlaps(self, v: Self) -> bool {
        self.complete_overlap(&v) || v.complete_overlap(&self)
    }
    fn some_overlap(&self, v: &Self) -> bool {
        self.0 <= v.0 && v.0 <= self.1 || self.0 <= v.1 && v.1 <= self.1
    }
    fn either_has_some_overlaps(self, v: Self) -> bool {
        self.some_overlap(&v) || v.some_overlap(&self)
    }
}

fn range(r: &(&str, &str)) -> Range {
    Range(str::parse(r.0).unwrap(), str::parse(r.1).unwrap()) // don't use unwrap in production here it helps debugging
}

fn part_1(src: &str) -> usize {
    src.lines()
        .filter_map(|l| l.split_once(','))
        .filter_map(|(x, y)| Some((x.split_once('-')?, y.split_once('-')?)))
        .filter(|(f, s)| range(f).either_complete_overlaps(range(s)))
        .count()
}

fn part_2(src: &str) -> usize {
    src.lines()
        .filter_map(|l| l.split_once(','))
        .filter_map(|(x, y)| Some((x.split_once('-')?, y.split_once('-')?)))
        .filter(|(f, s)| range(f).either_has_some_overlaps(range(s)))
        .count()
}
