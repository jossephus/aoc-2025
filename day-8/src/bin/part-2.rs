#[derive(Debug)]
struct Pair {
    x: i32,
    y: i32,
    z: i32,
}

struct UnionFind {
    par: Vec<usize>,
    size: Vec<u64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect::<Vec<_>>(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while self.par[x] != x {
            self.par[x] = self.par[self.par[x]];
            x = self.par[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let par_x = self.find(x);
        let par_y = self.find(y);

        if par_x == par_y {
            return false;
        }

        if self.size[par_x] >= self.size[par_y] {
            self.par[par_y] = par_x;
            self.size[par_x] += self.size[par_y];
        } else {
            self.par[par_x] = par_y;
            self.size[par_y] += self.size[par_x];
        }

        true
    }
}

impl Pair {
    fn distance(&self, other: &Pair) -> u64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

fn main() {
    let data = include_str!("../../input.txt");
    let pairs = data
        .lines()
        .map(|line| {
            let splitted = line.split(',').collect::<Vec<_>>();
            Pair {
                x: splitted.first().unwrap().parse::<i32>().unwrap(),
                y: splitted.get(1).unwrap().parse::<i32>().unwrap(),
                z: splitted.get(2).unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let n = pairs.len();
    let mut distances: Vec<(u64, usize, usize)> = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let curr_dist = pairs[i].distance(&pairs[j]);
            distances.push((curr_dist, i, j));
        }
    }

    let mut uf = UnionFind::new(n);
    distances.sort_by_key(|a| a.0);

    let mut last_x1 = 0;
    let mut last_x2 = 0;
    for (_, first_ind, second_ind) in distances {
        if uf.union(first_ind, second_ind) {
            last_x1 = pairs[first_ind].x as i64;
            last_x2 = pairs[second_ind].x as i64;
        }
    }

    println!("{}", last_x1 * last_x2);
}
