#[derive(Debug)]
struct Range {
    first_id: u64,
    last_id: u64,
}

impl Range {
    fn find_invalid_id(&self) -> Vec<String> {
        let mut invalid_ids = vec![];

        for i in self.first_id..=self.last_id {
            let data = format!("{}", i);
            let first_half = data[0..data.len() / 2].to_owned();
            let second_half = data[data.len() / 2..].to_owned();

            if first_half == second_half {
                invalid_ids.push(data);
            }
        }

        println!("{:?}", invalid_ids);
        invalid_ids
    }
}

fn main() {
    let data = include_str!("../../input.txt");
    let ranges = data
        .trim()
        .split(|line| line == ',')
        .map(|line| Range {
            first_id: (&line[0..line.find(|x| x == '-').unwrap()])
                .to_owned()
                .trim()
                .parse()
                .unwrap(),
            last_id: (&line[line.find(|x| x == '-').unwrap() + 1..])
                .to_owned()
                .trim()
                .parse()
                .unwrap(),
        })
        .collect::<Vec<_>>();

    let invalids: Vec<u64> = ranges
        .iter()
        .map(|x| x.find_invalid_id())
        .flatten()
        .inspect(|x| println!("{}", x))
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    //println!("Hello World {:?}", invalids.iter());
    println!("Hello World {:?}", invalids.iter().sum::<u64>());
}
