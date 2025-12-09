#[derive(Clone, Copy, Debug)]
struct Point {
    x: i128,
    y: i128,
}

impl Point {
    fn area(&self, other: Point) -> i128 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

fn main() {
    let data = include_str!("../input.txt");

    let points = data.lines().map(|x| {
        let split = x
            .split(",")
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<_>>();
        return Point {
            x: *split.get(0).unwrap(),
            y: *split.get(1).unwrap(),
        };
    });

    let mut largest = 0;

    points.clone().for_each(|first| {
        points.clone().for_each(|last| {
            if first.area(last) > largest {
                println!("between {:?} and {:?}", first, last);
                largest = first.area(last);
            }
        });
    });
    println!("Largest: {}", largest);
}
