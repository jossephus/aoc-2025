#[derive(Debug)]
struct Grid {
    position: (usize, usize),
    c: char,
}

fn main() {
    let data = include_str!("../../input.txt");

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let data = data
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| Grid {
                    position: (x, y),
                    c: c,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let sum: usize = data
        .iter()
        .filter(|x| x.c == '@')
        .map(|grid| {
            let count = dirs
                .iter()
                .filter(|(dx, dy)| {
                    let gx = grid.position.0;
                    let gy = grid.position.1;

                    let nx = gx as isize + dx;
                    let ny = gy as isize + dy;

                    println!("{} {} {} {}: {} {}", dx, dy, gx, gy, nx, ny,);

                    if nx < 0 || ny < 0 {
                        return false;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;

                    data.iter().any(|g| g.position == (nx, ny) && g.c == '@')
                })
                .count();

            if count < 4 { 1 } else { 0 }
        })
        .sum();

    println!("{:?}", sum);
}
