#[derive(Debug, Clone)]
struct Range {
    start: u128,
    end: u128,
}

fn main() {
    let ranges = include_str!("../../ranges.txt");

    let mut ranges = ranges
        .lines()
        .map(|line| Range {
            start: (&line[0..line.find(|x| x == '-').unwrap()])
                .to_owned()
                .trim()
                .parse()
                .unwrap(),
            end: (&line[line.find(|x| x == '-').unwrap() + 1..])
                .to_owned()
                .trim()
                .parse()
                .unwrap(),
        })
        .collect::<Vec<_>>();

    ranges.sort_by_key(|r| r.start);

    let mut merged = vec![ranges[0].clone()];

    for r in &ranges[1..] {
        let last = merged.last_mut().unwrap();
        if r.start <= last.end {
            last.end = last.end.max(r.end);
        } else {
            merged.push(r.clone());
        }
    }

    let total: u128 = merged.iter().map(|r| r.end - r.start + 1).sum();
    println!("{}", total);
}
