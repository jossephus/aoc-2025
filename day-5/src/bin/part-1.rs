#[derive(Debug)]
struct Range {
    start: u128,
    end: u128,
}

fn main() {
    let ranges = include_str!("../../ranges.txt");
    let ingredients = include_str!("../../ingredients.txt");

    let ranges = ranges
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

    let result = ingredients
        .lines()
        .filter(|x| {
            let ingredient = x.parse::<u128>().unwrap();

            let a = ranges
                .iter()
                .filter(|range| (range.start..=range.end).into_iter().contains(&ingredient))
                .collect::<Vec<_>>();

            !a.is_empty()
        })
        .collect::<Vec<_>>();

    println!("{:?}", result.iter().count());
}
