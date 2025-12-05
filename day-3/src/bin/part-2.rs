fn get_largest(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();
    let capacity = 12;
    let mut bucket: Vec<char> = Vec::with_capacity(capacity);

    for (i, &item) in chars.iter().enumerate() {
        let remaining = n - i;

        if bucket.is_empty() {
            bucket.push(item);
        } else {
            if item > bucket[bucket.len() - 1] {
                while !bucket.is_empty()
                    && bucket[bucket.len() - 1] < item
                    && bucket.len() - 1 + remaining >= capacity
                {
                    bucket.pop();
                }
                bucket.push(item);
            } else {
                if bucket.len() < capacity {
                    bucket.push(item);
                }
            }
        }
    }

    bucket.iter().collect()
}

/*
// Original approach (didn't work - was trying to replace elements in place instead of popping)
fn get_largest_original(line: &str) -> i64 {
    use std::cmp::max;

    let mut bucket: Vec<char> = Vec::with_capacity(12);
    let capacity = 12;

    line.chars().enumerate().for_each(|(i, item)| {
        if bucket.len() > 0 {
            // we find the last element
            let mut last_item = bucket[bucket.len() - 1];

            if item <= last_item {
                bucket.push(item);
            } else {
                let mut index = bucket.len() - 1;

                println!("{:?} {} {}", bucket, capacity, i);

                let a = max(capacity, i);

                println!("{} {}", index, a);

                if a > 0 {
                    while index <= a && last_item < item {
                        bucket[index] = item;

                        last_item = bucket.clone().into_iter().nth(index).unwrap();

                        if index > 0 {
                            index = index - 1;
                        }
                    }
                }
            }
        } else {
            bucket.push(item);
        }
    });
    println!("Bucket: {:?}", bucket);

    0
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(get_largest("987654321111111"), "987654321111");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(get_largest("811111111111119"), "811111111119");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(get_largest("234234234234278"), "434234234278");
    }

    #[test]
    fn test_case_4() {
        assert_eq!(get_largest("818181911112111"), "888911112111");
    }
}

fn main() {
    let data = include_str!("../../input.txt");

    let result: u64 = data
        .lines()
        .map(|line| get_largest(line).parse::<u64>().unwrap())
        .sum();

    println!("{}", result);
}
