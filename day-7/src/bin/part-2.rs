/// This is my first version of the solution
/// goes along with the solution for part-1 and works perfectly for sample input
/// but slower for part-2.
/// I wrote the below working version with the help of an LLM so to get the answer
/*
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Splitter {
    position: (usize, usize),
}

fn main() {
    let data = include_str!("../../input.txt");
    let mut starter: (usize, usize) = (0, 0);

    let splittrs = data
        .lines()
        .enumerate()
        .filter_map(|(x, line)| {
            let splitters = line
                .chars()
                .enumerate()
                .filter_map(|(y, c)| {
                    if c == 'S' {
                        starter = (x, y);
                    }
                    if c == '^' {
                        return Some(Splitter { position: (x, y) });
                    }
                    None
                })
                .collect::<Vec<_>>();

            Some(splitters)
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut queue: VecDeque<Splitter> = VecDeque::new();

    queue.push_back(Splitter { position: starter });

    while let Some(Splitter { position }) = queue.pop_front() {
        if let Some(splitter) = splittrs
            .iter()
            .find(|s| s.position.0 > position.0 && s.position.1 == position.1)
        {
            let left = Splitter {
                position: (splitter.position.0, splitter.position.1 - 1),
            };
            let right = Splitter {
                position: (splitter.position.0, splitter.position.1 + 1),
            };

            queue.push_back(left);
            queue.push_back(right);
        } else {
            println!("Counting {}", count);
            count += 1;
        }
    }

    println!("{}", count);
}
*/

fn main() {
    let data = include_str!("../../input.txt");
    let lines: Vec<&str> = data.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut table: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '^' => -1,
                    'S' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect();

    for i in 1..height {
        for j in 0..width {
            if table[i][j] >= 0 {
                table[i][j] += table[i - 1][j];
            } else if table[i][j] == -1 {
                let above = table[i - 1][j];
                table[i][j] = 0;
                if j > 0 {
                    table[i][j - 1] += above;
                }
                if j < width - 1 {
                    table[i][j + 1] += above;
                }
            }
        }
    }

    let result: i64 = table[height - 1].iter().sum();
    println!("{}", result);
}
