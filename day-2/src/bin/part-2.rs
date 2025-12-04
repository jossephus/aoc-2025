// 45283684555
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

            if self.is_repeated_pattern(&data) {
                invalid_ids.push(data);
            }
        }

        println!("{:?}", invalid_ids);
        invalid_ids
    }

    fn is_repeated_pattern(&self, s: &str) -> bool {
        let len = s.len();

        for pattern_len in 1..=len / 2 {
            if len % pattern_len == 0 {
                let pattern = &s[0..pattern_len];
                let repeats = len / pattern_len;

                if repeats >= 2 {
                    let constructed = pattern.repeat(repeats);
                    if constructed == s {
                        return true;
                    }
                }
            }
        }

        false
    }

    // based on amp's optimal solution
    // fn is_repeated_pattern(&self, s: &str) -> bool {
    //   (s.to_string() + s)[1..s.len() * 2 - 1].contains(s)
    // }
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
