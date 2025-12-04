struct Grid {
    position: (usize, usize),
    c: char,
}

fn remove_positions(grids: &[Grid]) -> Vec<(usize, usize)> {
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

    let at_positions: std::collections::HashSet<_> = grids
        .iter()
        .filter(|g| g.c == '@')
        .map(|g| g.position)
        .collect();

    grids
        .iter()
        .filter(|x| x.c == '@')
        .filter_map(|grid| {
            let (gx, gy) = grid.position;

            let count = dirs
                .iter()
                .filter(|(dx, dy)| {
                    let nx = gx as isize + dx;
                    let ny = gy as isize + dy;

                    if nx < 0 || ny < 0 {
                        return false;
                    }

                    let (nx, ny) = (nx as usize, ny as usize);

                    at_positions.contains(&(nx as usize, ny as usize))
                })
                .count();

            if count >= 4 {
                return None;
            }

            Some(vec![grid.position])
        })
        .flatten()
        .collect()
}

fn main() {
    let data = include_str!("../../input.txt");

    let mut grids = data
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| Grid {
                    position: (x, y),
                    c,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_removed = 0;
    let mut i = 1;

    loop {
        let removable = remove_positions(&grids);

        if removable.is_empty() {
            break;
        }

        println!("{}", i);
        i = i + 1;

        total_removed += removable.len();

        for grid in &mut grids {
            if removable.contains(&grid.position) {
                grid.c = 'x';
            }
        }
    }

    println!("Total rolls removed: {}", total_removed);
}
