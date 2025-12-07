use std::collections::{HashSet, VecDeque};

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
    let mut visited: HashSet<Splitter> = HashSet::new();
    let mut split_counter: HashSet<Splitter> = HashSet::new();

    queue.push_back(Splitter { position: starter });
    visited.insert(Splitter { position: starter });

    while let Some(Splitter { position }) = queue.pop_front() {
        if let Some(splitter) = splittrs
            .iter()
            .find(|s| s.position.0 > position.0 && s.position.1 == position.1)
        {
            if split_counter.insert(*splitter) {
                count += 1;
            }

            let left = Splitter {
                position: (splitter.position.0, splitter.position.1 - 1),
            };
            let right = Splitter {
                position: (splitter.position.0, splitter.position.1 + 1),
            };

            if visited.insert(left) {
                queue.push_back(left);
            }
            if visited.insert(right) {
                queue.push_back(right);
            }
        }
    }

    println!("{}", count);
}
/*
(0, 7)
(2, 7)
(4, 6) (4, 8)
(6, 5) (6, 7) (6, 9)
(8, 4) (8, 6) (8, 10)
(10, 3) (10, 5) (10, 9) (10, 11)
*/
