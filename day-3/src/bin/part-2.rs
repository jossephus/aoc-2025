use std::cmp::max;

fn get_largest(line: &str) -> i64 {
    // 9869755494885236452767647548826258555668838497687557968652485746757835445145546687436356799481798587

    let mut bucket: Vec<char> = Vec::with_capacity(12);

    line.chars().enumerate().for_each(|(i, item)| {
        if bucket.len() > 0 {
            // we find the last element
            let mut last_item = bucket.clone().into_iter().nth(bucket.len() - 1).unwrap();

            if item < last_item {
                bucket.push(item);
            } else {
                let mut index = bucket.len() - 1;

                println!("{:?} {} {}", bucket, bucket.capacity(), i);
                //println!("{}  {}", bucket.capacity(), line.len() - i);
                //
                let a = max(0, bucket.len() - (line.len() - i));

                if a > 0 {
                    while index > a && last_item > item {
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

fn main() {
    let data = include_str!("../../input.txt");

    let sum: i64 = data
        .lines()
        .enumerate()
        .map(|(_, line)| {
            let large = get_largest(line);
            large
        })
        .into_iter()
        .sum();
    println!("Hello World {}", sum);
}
