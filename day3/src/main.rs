use itertools::*;
use std::collections::HashSet;

fn main() {
    println!("--------AOC-2022-DAY-3-------");
    let src = std::fs::read_to_string("./day3/input").unwrap_or_default();
    println!("P1-> {}", part_1(&src));
    println!("P2-> {}", part_2(&src));
}

fn str_to_hashset(src: &str) -> HashSet<u8> {
    src.as_bytes()
        .into_iter()
        .fold(HashSet::new(), |mut set, &i| {
            set.insert(i);
            set
        })
}

fn get_priority(ch: u8) -> usize {
    (if b'A' <= ch && b'Z' >= ch {
        ch - b'A' + 27
    } else {
        ch - b'a' + 1
    }) as usize
}

fn part_1(src: &str) -> usize {
    src.lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(x, y)| (str_to_hashset(x), str_to_hashset(y)))
        .filter_map(|(x, ref y)| x.intersection(y).next().copied())
        .map(get_priority)
        .sum()
}

fn part_2(src: &str) -> usize {
    src.lines()
        .tuples::<(&str, &str, &str)>()
        .map(|(x, y, z)| (str_to_hashset(x), str_to_hashset(y), str_to_hashset(z)))
        .filter_map(|(x, y, z)| {
            x.intersection(&y.intersection(&z).copied().collect())
                .next()
                .copied()
        })
        .map(get_priority)
        .sum()
}
