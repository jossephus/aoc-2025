use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Group {
    nums: Vec<char>,
}

#[derive(Debug, Clone)]
struct SuperGroup {
    groups: Vec<Group>,
    operator: Option<char>,
}

fn main() {
    let mut map: HashMap<(usize, usize), Group> = HashMap::new();

    let data = include_str!("../../input.txt");

    data.lines().enumerate().for_each(|(tp_index, line)| {
        line.chars().rev().enumerate().for_each(|(index, x)| {
            print!("{}", x);
            map.entry((tp_index, index))
                .and_modify(|v| {
                    v.nums.push(x);
                })
                .or_insert_with(|| {
                    return Group { nums: vec![x] };
                });
        });
    });
    print!("\n");

    let mut nums = map.iter().collect::<Vec<_>>();
    nums.sort_by_key(|((line, _), _)| line);

    //let nums: Vec<_> = nums
    //.iter()
    //.map(|(_, x)| &x.nums[..])
    //.map(|x| x)
    //.flatten()
    //.collect();

    println!("{:?}", nums);

    //let sum: u64 = map.iter().map(|(_, group)| group.apply()).sum();

    //println!("{:?}", sum);
}
