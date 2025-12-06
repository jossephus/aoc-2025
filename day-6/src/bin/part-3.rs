use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Group {
    nums: Vec<char>,
}

#[derive(Debug, Clone)]
struct SuperGroup {
    nums: Vec<Vec<char>>,
    operator: Option<char>,
}

impl SuperGroup {
    fn sum(&self) -> u64 {
        if let Some(op) = self.operator {
            return self
                .nums
                .iter()
                .map(|vec| {
                    vec.iter()
                        .filter(|&&c| c != ' ' && c != '+' && c != '*')
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap_or(if op == '+' { 0 } else { 1 })
                })
                .fold(if op == '+' { 0 } else { 1 }, |x, y| {
                    return if op == '+' { x + y } else { x * y };
                });
        }
        0
    }
}

fn main() {
    let mut map: HashMap<(usize, usize), Group> = HashMap::new();

    let data = include_str!("../../input.txt");

    data.lines().enumerate().for_each(|(tp_index, line)| {
        line.chars().rev().enumerate().for_each(|(index, x)| {
            map.entry((tp_index, index))
                .and_modify(|v| {
                    v.nums.push(x);
                })
                .or_insert_with(|| {
                    return Group { nums: vec![x] };
                });
        });
    });

    let mut columns: Vec<usize> = map.keys().map(|&(_, col)| col).collect();
    columns.sort_unstable_by(|a, b| b.cmp(a));
    columns.dedup();

    let mut groups: Vec<SuperGroup> = vec![];

    for col in columns {
        let mut nums: Vec<_> = map.iter().filter(|(x, _)| x.1 == col).collect();
        nums.sort_by_key(|((line, _), _)| line);

        let nums: Vec<char> = nums.iter().flat_map(|(_, x)| &x.nums).copied().collect();

        let operator = nums.iter().find(|&&c| c == '+' || c == '*').copied();

        if operator.is_some() {
            groups.push(SuperGroup {
                nums: vec![nums],
                operator,
            });
        } else {
            let idx = groups.len() - 1;
            groups[idx].nums.push(nums);
        }
    }

    let total: u64 = groups.iter().map(SuperGroup::sum).sum();

    println!("{:?}", total);

    //let nums: Vec<_> = nums
    //.iter()
    //.map(|(_, x)| &x.nums[..])
    //.map(|x| x)
    //.flatten()
    //.collect();

    //let sum: u64 = map.iter().map(|(_, group)| group.apply()).sum();
}
