#[derive(Debug)]
enum Direction {
    Left(i32),
    Right(i32),
}

struct Dialer {
    password: i32,
    value: i32,
}

impl Dialer {
    fn new(value: i32) -> Self {
        Dialer { value, password: 0 }
    }

    fn rotate(&mut self, direction: &Direction) {
        match direction {
            Direction::Left(value) => {
                if self.value - value < 0 {
                    self.value = (self.value - value % 100 + 100) % 100;
                } else {
                    self.value -= value;
                }
            }
            Direction::Right(value) => {
                if self.value + value >= 100 {
                    self.value = (self.value + value) % 100;
                } else {
                    self.value += value;
                }
            }
        }

        if self.value == 0 {
            self.password += 1;
        }
    }
}

fn main() {
    let data = include_str!("../../input.txt").lines();

    let mut dialer = Dialer::new(50);

    let directions = data
        .into_iter()
        .map(|line| {
            if line.chars().next().unwrap() == 'L' {
                return Direction::Left(*&line[1..].parse().unwrap());
            } else {
                return Direction::Right(*&line[1..].parse().unwrap());
            }
        })
        .collect::<Vec<_>>();

    directions.iter().for_each(|direction| {
        dialer.rotate(direction);
    });
    println!("Dialer Password is {}", dialer.password);
}
