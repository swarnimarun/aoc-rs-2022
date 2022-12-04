fn main() {
    println!("--------AOC-2022-DAY-2-------");
    let src = std::fs::read_to_string("./day2/input").unwrap_or_default();
    println!("P1-> {}", part_1(&src));
    println!("P2-> {}", part_2(&src));
}

fn decide_score<T: From<u8>>((o, m): (u8, u8)) -> T {
    let win_score = match (o, m) {
        (b'C', b'X') | (b'A', b'Y') | (b'B', b'Z') => 6,
        (b'A', b'X') | (b'B', b'Y') | (b'C', b'Z') => 3,
        _ => 0,
    };
    (m - b'X' + 1 + win_score).into()
}

fn part_1(src: &str) -> usize {
    src.lines()
        .filter_map(|x| x.split_once(' '))
        .map(|(x, y)| (x.as_bytes()[0], y.as_bytes()[0]))
        .map(decide_score::<usize>)
        .sum()
}

fn pick_play((opponent, r): (u8, u8)) -> (u8, u8) {
    let o = opponent - b'A';
    let me = match r {
        b'X' => (o + 2) % 3 + b'X', // lose
        b'Y' => o + b'X', // draw
        _ => (o + 1) % 3 + b'X', // win
    };
    (opponent, me)
}

fn part_2(src: &str) -> usize {
    src.lines()
        .filter_map(|x| x.split_once(' '))
        .map(|(x, y)| (x.as_bytes()[0], y.as_bytes()[0]))
        .map(pick_play)
        .map(decide_score::<usize>)
        .sum()
}
