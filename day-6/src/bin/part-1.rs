use std::collections::HashMap;

#[derive(Debug)]
struct Group {
    nums: Vec<u64>,
    operator: Option<char>,
}

impl Group {
    fn apply(&self) -> u64 {
        let mut total = 0;

        match self.operator {
            Some(operator) => {
                if operator == '+' {
                    total = self.nums.iter().sum();
                } else if operator == '*' {
                    total = self.nums.iter().fold(1, |x, y| x * y);
                }
            }
            None => {}
        }
        return total;
    }
}

fn main() {
    let mut map: HashMap<usize, Group> = HashMap::new();

    let data = include_str!("../../input.txt");

    data.lines().for_each(|x| {
        x.split_whitespace().enumerate().for_each(|(index, x)| {
            map.entry(index)
                .and_modify(|v| {
                    if let Ok(num) = x.parse::<u64>() {
                        v.nums.push(num);
                    } else {
                        println!("Hello {:?}", x);
                        v.operator = Some(x.parse::<char>().unwrap());
                    }
                })
                .or_insert_with(|| {
                    return Group {
                        nums: vec![x.parse::<u64>().unwrap()],
                        operator: None,
                    };
                });
        });
    });

    let sum: u64 = map.iter().map(|(_, group)| group.apply()).sum();

    println!("{:?}", sum);
}
