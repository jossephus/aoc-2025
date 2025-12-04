fn get_largest(line: &str) -> u64 {
    let mut largest: u64 = 0;

    line.chars().enumerate().for_each(|(index, a)| {
        line[index + 1..].chars().for_each(|b| {
            let num = format!("{}{}", a, b);
            if num.parse::<u64>().unwrap() > largest {
                largest = num.parse().unwrap();
            }
        });
    });

    largest
}

fn main() {
    let data = include_str!("../../input.txt");

    let sum: u64 = data.lines().map(|line| get_largest(line)).into_iter().sum();
    println!("Hello World {}", sum);
}
