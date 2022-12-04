use itertools::Itertools;

fn main() {
    println!("--------AOC-2022-DAY-1-------");
    let src = std::fs::read_to_string("./day1/input").unwrap_or_default();
    println!("P1-> {}", part_1(&src));
    println!("P2-> {}", part_2(&src));
}

fn part_1<S: AsRef<str>>(src: S) -> usize {
    src.as_ref()
        .lines()
        .map(str::parse::<usize>)
        .group_by(|x| x.is_ok())
        .into_iter()
        .map(|(_, x)| x.flatten().sum())
        .max()
        .unwrap_or_default()
}

fn part_2<S: AsRef<str>>(src: S) -> usize {
    src.as_ref()
        .lines()
        .map(str::parse::<usize>)
        .group_by(|x| x.is_ok())
        .into_iter()
        .map(|(_, x)| x.flatten().sum())
        .sorted_by(|x: &usize, y| y.cmp(x))
        .take(3)
        .sum()
}
